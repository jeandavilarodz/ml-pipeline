//! This module contains logic to pre-process the input data for later use

mod mean;

use crate::data::column::Column;
use crate::data::data_frame::DataFrame;
use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

use lazy_static::lazy_static;

trait Scrubber {
    fn clean(&self, column: &mut Column<Option<Numeric>>);
}

lazy_static! {
    static ref SCRUBBER_TYPE_MAP: HashMap<&'static str, Box<dyn Scrubber + Sync>> =
        HashMap::from([(
            "mean",
            Box::new(mean::MeanScrubber) as Box<dyn Scrubber + Sync>
        ),]);
}

pub fn scrub(
    table: DataFrame<Option<Numeric>>,
    scrubbers: Vec<(&str, usize)>,
) -> Result<DataFrame<Numeric>, Box<dyn Error>> {
    let missing_parser = scrubbers
        .iter()
        .fold(false, |acc, p| acc | !SCRUBBER_TYPE_MAP.contains_key(p.0));

    if missing_parser {
        return Err("Scrubber type not supported!".into());
    }

    let mut table = table;

    for (scrubber, idx) in scrubbers.into_iter() {
        let scrubber = &SCRUBBER_TYPE_MAP[scrubber];
        if let Some(column) = table.get_column_idx_mut(idx) {
            scrubber.clean(column);
        }
    }

    // Calculate the amputation indexes and sort them in order to use iteration to remove the values
    let mut amputation_index = Vec::new();
    for column in table.columns() {
        for (idx, val) in column.values().enumerate() {
            if val.is_none() {
                amputation_index.push(idx);
            }
        }
    }
    amputation_index.sort();

    let mut ret = DataFrame::new();

    for column in table.columns_mut() {
        for idx in amputation_index.iter().rev() {
            column.remove(*idx);
        }
        let mut clean_col = Column::new();
        if let Some(name) = column.get_name() {
            clean_col.set_name(name.to_owned());
        }
        if let Some(metadata) = column.get_metadata() {
            clean_col.set_metadata(metadata.to_owned());
        }
        clean_col.append(column.values().filter_map(|x| *x).collect());
        ret.add_column(clean_col);
    }

    Ok(ret)
}
