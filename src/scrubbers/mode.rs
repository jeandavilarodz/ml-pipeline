//! Scrubber that replaces the missing values with the most common value

use super::Scrubber;
use crate::data::column::Column;
use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

pub struct ModeScrubber;

impl Scrubber for ModeScrubber {
    fn clean(column: &mut Column<Option<Numeric>>) -> Result<(), Box<dyn Error>> {
        let mut label_count = HashMap::new();

        for value in column.values().filter_map(|&value| value) {
            let key = (value * 1e8) as i64;
            let counter = label_count.entry(key).or_insert(0);
            *counter += 1;
        }

        let mode = label_count
                                .iter()
                                .max_by_key(|&(_, count)| count)
                                .map(|(val, _)| val);
        
        if mode.is_none() {
            return Err("Mode of column not found!".into());
        }

        let mode = (*mode.unwrap() as f64) * 1e-8;

        column.values_mut().for_each(|v| *v = v.or(Some(mode)));

        Ok(())
    }
}
