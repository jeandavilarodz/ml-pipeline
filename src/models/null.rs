// null_regression.rs

//! This module implements a dummy regression model that predicts trends as the average of all
//! output values in the original training data set.

use super::Model;
use super::ModelBuilder;

use num_traits::ToPrimitive;

use crate::types::{Numeric, NUMERIC_DIGIT_PRECISION};

use std::collections::HashMap;
use std::error::Error;

pub struct NullModel {
    return_value: Numeric,
}

impl Model for NullModel {
    fn predict(&self, _sample: &[Numeric]) -> Numeric {
        self.return_value
    }
}

pub struct NullRegressionModelTrainer;

impl ModelBuilder for NullRegressionModelTrainer {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn with_parameters(
        &mut self,
        _parameters: &Option<HashMap<String, Numeric>>,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn with_features(&mut self, _features: &[Box<[Numeric]>]) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn build(
        &mut self,
        training_values: &[Box<[Numeric]>],
        target_value_idx: usize,
    ) -> Result<Box<dyn Model>, Box<dyn Error>> {
        // Calculate mean of labels
        let mean = training_values.iter().fold(0.0, |acc, val| {
            acc + val[target_value_idx].to_f64().unwrap()
        }) / training_values.len() as f64;

        Ok(Box::new(NullModel { return_value: mean }))
    }
}

pub struct NullClassificationModelTrainer;

impl ModelBuilder for NullClassificationModelTrainer {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn with_parameters(
        &mut self,
        _parameters: &Option<HashMap<String, Numeric>>,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn with_features(&mut self, _features: &[Box<[Numeric]>]) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn build(
        &mut self,
        training_values: &[Box<[Numeric]>],
        target_value_idx: usize,
    ) -> Result<Box<dyn Model>, Box<dyn Error>> {
        // Build a map of counters for the most common value
        let mut value_count = HashMap::new();

        // Populate the map with the counts of the most common values
        for value in training_values.iter() {
            let key = (value[target_value_idx] / NUMERIC_DIGIT_PRECISION)
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

        Ok(Box::new(NullModel {
            return_value: (*mode as f64) * NUMERIC_DIGIT_PRECISION,
        }))
    }
}
