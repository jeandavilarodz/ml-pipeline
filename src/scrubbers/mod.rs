//! This module contains logic to pre-process the input data for later use

mod mean;
mod mode;

use crate::config::ScrubbingStageConfigs;
use crate::data::data_frame::DataFrame;
use crate::data::column::Column;
use crate::types::Numeric;

use std::error::Error;

trait Scrubber {
    fn clean(column: &mut Column<Option<Numeric>>) -> Result<(), Box<dyn Error>>;
}

type ScrubFnPtr = fn(&mut Column<Option<Numeric>>) -> Result<(), Box<dyn Error>>;

pub fn get_scrubber(name: &str) -> Result<ScrubFnPtr, Box<dyn Error>> {
    match name {
        "mean" => Ok(mean::MeanScrubber::clean),
        "mode" => Ok(mode::ModeScrubber::clean),
        _ => Err("Invalid scrubber name given!".into()),
    }
}

pub fn scrub(
    table: DataFrame<Option<Numeric>>,
    parameters: Vec<ScrubbingStageConfigs>,
) -> Result<DataFrame<Numeric>, Box<dyn Error>> {
    let mut table = table;

    for parameter in parameters.into_iter() {
        let scrubber = get_scrubber(&parameter.name)?;
        if let Some(column) = table.get_column_idx_mut(parameter.index) {
            scrubber(column)?;
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
