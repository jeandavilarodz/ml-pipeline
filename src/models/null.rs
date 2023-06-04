use super::Model;
use super::ModelFactory;

use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

pub struct NullModel {
    value_to_replace: Numeric,
}

impl Model for NullModel {
    fn predict(&self, _sample: &[Numeric]) -> Result<Numeric, Box<dyn Error>> {
        Ok(self.value_to_replace)
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
            let counter = value_count.entry(value as i64).or_insert(0);
            *counter += 1;
        }

        let mode = value_count
            .iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .ok_or("No mode found!")?;

        Ok(Box::new(NullModel {
            value_to_replace: *mode as Numeric,
        }))
    }
}
