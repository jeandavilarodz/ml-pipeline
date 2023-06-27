// null_regression.rs

//! This module implements a dummy regression model that predicts trends as the average of all
//! output values in the original training data set.

use super::Model;
use super::ModelBuilder;

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

    fn label(&self, _sample: &[Numeric]) -> Numeric {
        self.return_value
    }
    fn type_id(&self) -> &'static str {
        "NullModel"
    }

    fn get_hyperparameters(&self) -> HashMap<String, String> {
        HashMap::from([("return_value".into(), self.return_value.to_string())])
    }
    fn set_hyperparameters(&mut self, hyperparameters: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
        if let Some(return_value) = hyperparameters.get("return_value") {
            self.return_value = return_value.parse::<Numeric>()?;
        }
        Ok(())
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

    fn with_hyperparameters(&mut self, _features: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn build(
        &mut self,
        training_values: &[Box<[Numeric]>],
        target_value_idx: usize,
    ) -> Result<Box<dyn Model>, Box<dyn Error>> {
        // Calculate mean of labels
        let mean = training_values
            .iter()
            .fold(0.0, |acc, val| acc + val[target_value_idx])
            / (training_values.len() as f64);

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

    fn with_hyperparameters(&mut self, _features: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
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
        training_values.iter().for_each(|sample| {
            let key = (sample[target_value_idx] / NUMERIC_DIGIT_PRECISION) as i64;
            let counter = value_count.entry(key).or_insert(0);
            *counter += 1;
        });

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
