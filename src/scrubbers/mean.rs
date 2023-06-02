//! Scrubber that replaces the missing values with the mean of the present values

use super::Scrubber;
use crate::data::column::Column;
use crate::types::Numeric;

pub struct MeanScrubber;

impl Scrubber for MeanScrubber {
    fn clean(&self, column: &mut Column<Option<Numeric>>) {
        let sum: Numeric = column.values().filter_map(|&value| value).sum();
        let count = column.values().filter_map(|&value| value).count();

        if count == 0 {
            return;
        }

        let mean = sum / (count as Numeric);

        column.values_mut().for_each(|v| *v = v.or(Some(mean)));
    }
}
