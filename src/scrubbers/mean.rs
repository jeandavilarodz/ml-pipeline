//! Scrubber that replaces the missing values with the mean of the present values

use super::Scrubber;
use crate::data::column::Column;
use crate::types::Numeric;

pub struct MeanScrubber;

impl Scrubber for MeanScrubber {
    fn clean(&self, column: &mut Column<Option<Numeric>>) {
        let sum: f32 = column.values().filter_map(|&value| value).sum();
        let count = column.values().filter(|&value| value.is_some()).count();

        if count == 0 {
            return;
        }

        let mean = sum / (count as f32);

        column.values_mut().for_each(|v| *v = v.or(Some(mean)));
    }
}
