use num_traits::ToPrimitive;

use super::Model;
use super::ModelFactory;

use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

pub struct NullModel {
    return_value: Numeric,
}

impl Model for NullModel {
    fn predict(&self, _sample: &Vec<&Numeric>) -> Result<Numeric, Box<dyn Error>> {
        Ok(self.return_value)
    }
}

pub struct NullModelFactory;

impl ModelFactory for NullModelFactory {
    fn build(
        &self,
        training_values: &Vec<Vec<&Numeric>>,
        target_value_idx: usize,
    ) -> Result<Box<dyn Model>, Box<dyn Error>> {
        let mut value_count = HashMap::new();

        for value in training_values.iter() {
            let key = (value[target_value_idx] * 1e8)
                .to_i64()
                .ok_or("Could not turn Numeric into key!")?;
            let counter = value_count.entry(key).or_insert(0);
            *counter += 1;
        }

        let mode = value_count
            .iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .ok_or("No mode found!")?;

        Ok(Box::new(NullModel {
            return_value: (1e-8 * (*mode as f64)),
        }))
    }
}

pub struct NullRegressionModelFactory;

impl ModelFactory for NullRegressionModelFactory {
    fn build(
        &self,
        training_values: &Vec<Vec<&Numeric>>,
        target_value_idx: usize,
    ) -> Result<Box<dyn Model>, Box<dyn Error>> {
        let mean = training_values.iter().fold(0.0, |acc, val| {
            acc + val[target_value_idx].to_f64().unwrap()
        }) / training_values.len() as f64;

        Ok(Box::new(NullModel { return_value: mean }))
    }
}
