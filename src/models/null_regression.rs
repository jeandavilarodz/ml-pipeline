// null_regression.rs

/// This module implements a dummy regression model that predicts trends as the average of all
/// output values in the original training data set.

use super::Model;

use num_traits::ToPrimitive;
use rand::Rng;

use crate::types::Numeric;

use std::error::Error;

pub struct NullRegressionModel {
    return_value: Numeric,
}

impl Model for NullRegressionModel {
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
        // Calculate mean of labels
        let mean = training_values.iter().fold(0.0, |acc, val| {
            acc + val[target_value_idx].to_f64().unwrap()
        }) / training_values.len() as f64;

        self.return_value = mean;

        Ok(())
    }
}
