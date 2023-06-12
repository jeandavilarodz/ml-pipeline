use super::Model;

use num_traits::ToPrimitive;
use rand::Rng;

use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

// null_classification.rs

/// This module implements a dummy classification model that labels samples as the most common label
/// seen in the input training data.

pub struct NullClassificationModel {
    return_value: Numeric,
}

impl Model for NullClassificationModel {
    fn new() -> Self {
        Self {
            return_value: rand::thread_rng().gen::<f64>(),
        }
    }

    fn predict(&self, _sample: &Vec<&Numeric>) -> Result<Numeric, Box<dyn Error>> {
        Ok(self.return_value)
    }

    fn train(
        &mut self,
        training_values: &Vec<Vec<&Numeric>>,
        target_value_idx: usize,
    ) -> Result<(), Box<dyn Error>> {
        // Build a map of counters for the most common value
        let mut value_count = HashMap::new();

        for value in training_values.iter() {
            let key = (value[target_value_idx] * 1e8)
                .to_i64()
                .ok_or("Could not turn Numeric into key!")?;
            let counter = value_count.entry(key).or_insert(0);
            *counter += 1;
        }

        // Grab the value of the counter with the largest value
        let mode = value_count
            .iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .ok_or("No mode found!")?;

        // Make the new return value be the mode of the new dataset
        self.return_value = 1e-8 * (*mode as f64);

        Ok(())
    }
}
