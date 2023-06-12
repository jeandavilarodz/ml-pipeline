// mode.rs

/// Scrubber that replaces the missing values with the most common value (mode)

use super::Scrubber;
use crate::data::column::Column;
use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

pub struct ModeScrubber;

impl Scrubber for ModeScrubber {
    fn clean(column: &mut Column<Option<Numeric>>) -> Result<(), Box<dyn Error>> {
        let mut label_count = HashMap::new();

        // Populate map with the count of unique values in the input set
        // Essentially, calculate the histogram of the input
        for value in column.values().filter_map(|&value| value) {
            let key = (value * 1e8) as i64;
            let counter = label_count.entry(key).or_insert(0);
            *counter += 1;
        }

        // Grab the maximum value count in the histrogram and return this
        let mode = label_count
                                .iter()
                                .max_by_key(|&(_, count)| count)
                                .map(|(val, _)| val)
                                .ok_or("No maximum found in the histogram!")?;
        
        // Convert histrogram key back into the original value to get the mode
        let mode = (*mode as f64) * 1e-8;

        // Replace every value in the column that is none with the mode
        // Note: the ".or" method in an Option type will replace the None option
        // by the given value, in this case Some(mode).
        column.values_mut().for_each(|v| *v = v.or(Some(mode)));

        Ok(())
    }
}
