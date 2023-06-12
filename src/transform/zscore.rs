// zscore.rs

/// This module implements the logic for z-score standardization where the mean and sample variance is
/// used to modify the range of values of the input data to be from -1 to 1

use std::collections::HashMap;

use super::Transform;
use crate::data::column::Column;
use crate::types::Numeric;

use std::error::Error;

pub struct ZScoreNormalization;

impl Transform for ZScoreNormalization {
    fn apply(
        column: &mut Column<Numeric>,
        _parameters: &Option<HashMap<String, Numeric>>,
    ) -> Result<(), Box<dyn Error>> {
        let sum = column.values().fold(0.0, |acc, &v| acc + v);
        let count = column.values().count();

        if count == 0 {
            return Err("Number of items in the column is zero!".into());
        }

        let mean = sum / (count as f64);

        let variance = column
            .values()
            .fold(0.0, |acc, &n| acc + (n - mean) * (n - mean))
            / ((count - 1) as f64);
        let std_deviation = variance.sqrt();

        for value in column.values_mut() {
            *value = (*value - mean) / std_deviation;
        }

        Ok(())
    }
}
