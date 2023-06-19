// null_regression.rs

//! This module implements a dummy regression model that predicts trends as the average of all
//! output values in the original training data set.

use super::ModelTrainer;
use super::Model;

use num_traits::ToPrimitive;

use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

pub struct NullModel {
    return_value: Numeric,
}

impl Model for NullModel {
    fn predict(&self, _sample: Box<[Numeric]>) -> Numeric {
        self.return_value
    }
}

pub struct NullRegressionModelTrainer {
    training_data: Option<Vec<Box<[Numeric]>>>,
    label_index: Option<usize>,
}

impl ModelTrainer for NullRegressionModelTrainer {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            training_data: None,
            label_index: None,
        }
    }

    fn with_parameters(
        &mut self,
        _parameters: &Option<HashMap<String, Numeric>>,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn with_training_data(
        &mut self,
        training_values: &Vec<Box<[Numeric]>>,
        label_idx: usize,
    ) -> Result<(), Box<dyn Error>> {
        if training_values.len() < 1 {
            return Err("Empty training set given!".into());
        }
        if training_values.get(label_idx).is_none() {
            return Err("Could not find target label in training data!".into());
        }
        self.training_data = Some(training_values.clone());
        self.label_index = Some(label_idx);
        Ok(())
    }

    fn train(&mut self) -> Result<Box<dyn Model>, Box<dyn Error>> {
        let training_data = self.training_data.as_ref().ok_or("No training data!")?;
        let label_index = self.label_index.ok_or("No label_index!")?;

        // Calculate mean of labels
        let mean = training_data.iter().fold(0.0, |acc, val| {
            acc + val[label_index].to_f64().unwrap()
        }) / training_data.len() as f64;

        Ok(Box::new(NullModel {
            return_value: mean,
        }))
    }
}

pub struct NullClassificationModelTrainer {
    training_data: Option<Vec<Box<[Numeric]>>>,
    label_index: Option<usize>,
}

impl ModelTrainer for NullClassificationModelTrainer {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            training_data: None,
            label_index: None,
        }
    }

    fn with_parameters(
        &mut self,
        _parameters: &Option<HashMap<String, Numeric>>,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn with_training_data(
        &mut self,
        training_values: &Vec<Box<[Numeric]>>,
        label_idx: usize,
    ) -> Result<(), Box<dyn Error>> {
        if training_values.len() < 1 {
            return Err("Empty training set given!".into());
        }
        if training_values.get(label_idx).is_none() {
            return Err("Could not find target label in training data!".into());
        }
        self.training_data = Some(training_values.clone());
        self.label_index = Some(label_idx);
        Ok(())
    }

    fn train(&mut self) -> Result<Box<dyn Model>, Box<dyn Error>> {
        let training_data = self.training_data.as_ref().ok_or("No training data!")?;
        let label_index = self.label_index.ok_or("No label_index!")?;

        // Build a map of counters for the most common value
        let mut value_count = HashMap::new();

        for value in training_data.into_iter() {
            let key = (value[label_index] * 1e8)
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
            return_value: 1e-8 * (*mode as f64),
        }))
    }
}
