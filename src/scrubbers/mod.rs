//! This module contains logic to pre-process the input data for later use
use crate::data::column::Column;

trait DataScrubber<T> {
    fn clean(column: Column<T>) -> Column<Option<f32>>;
}
