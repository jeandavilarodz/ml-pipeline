//! Scrubber that replaces the missing values with the mean of the present values

use super::Scrubber;
use crate::data::column::Column;
use crate::types::Numeric;

use std::error::Error;

pub struct MeanScrubber;

impl Scrubber for MeanScrubber {
    fn clean(column: &mut Column<Option<Numeric>>) -> Result<(), Box<dyn Error>> {
        // Sum over all elements that are not None
        let sum = column
            .values()
            .filter_map(|&value| value)
            .fold(0.0, |acc, v| acc + v);

        // Grab number of all elements that are not None
        let count = column.values().filter_map(|&value| value).count();

        if count == 0 {
            return Err("Null column passed to MeanScrubber!".into());
        }

        let mean = sum / (count as f64);

        column.values_mut().for_each(|v| *v = v.or(Some(mean)));
        
        Ok(())
    }
}
