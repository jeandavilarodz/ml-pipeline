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
    fn predict(&self, _sample: &[Numeric]) -> Result<Numeric, Box<dyn Error>> {
        Ok(self.return_value)
    }
}

pub struct NullModelFactory;

impl ModelFactory for NullModelFactory {
    fn from_training(
        &self,
        _training_values: &[&[Numeric]],
        target_values: &[Numeric],
    ) -> Result<Box<dyn Model>, Box<dyn Error>> {
        let mut value_count = HashMap::new();

        for &value in target_values {
            let key = (value * 1e8).to_i64().ok_or("Could not turn Numeric into key!")?;
            let counter = value_count.entry(key).or_insert(0);
            *counter += 1;
        }

        let mode = value_count
            .iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .ok_or("No mode found!")?;

        Ok(Box::new(NullModel {
            return_value: Numeric::from(1e-8 * (*mode as f64)),
        }))
    }
}

pub struct NullRegressionModelFactory;

impl ModelFactory for NullRegressionModelFactory {
    fn from_training(
        &self,
        _training_values: &[&[Numeric]],
        target_values: &[Numeric],
    ) -> Result<Box<dyn Model>, Box<dyn Error>> {
        let mean = target_values.iter().fold(0.0, |acc, &val| acc + val.to_f64().unwrap()) / target_values.len() as f64;

        Ok(Box::new(NullModel {
            return_value: Numeric::from(mean),
        }))
    }
}
