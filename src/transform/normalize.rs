use std::collections::HashMap;

use super::Transform;
use crate::data::column::Column;
use crate::types::Numeric;

pub struct NormalizeTransform;

impl Transform for NormalizeTransform {
    fn apply(&self, column: &mut Column<Numeric>, _parameters: &Option<HashMap<&str,Numeric>>) {
        let sum: Numeric = column.values().sum();
        let count = column.values().count();

        if count == 0 {
            return;
        }

        let mean = sum / (count as Numeric);

        let variance = column
            .values()
            .fold(0 as Numeric, |acc, &n| acc + (n - mean) * (n - mean))
            / (count - 1) as Numeric;
        let std_deviation = variance.sqrt();

        for value in column.values_mut() {
            *value = (*value - mean) / std_deviation;
        }
    }
}
