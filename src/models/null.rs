use super::Model;

use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

pub struct NullModel {
    value_to_replace: Numeric,
}

impl Model for NullModel {
    fn from_training(_training_values: &[&[Numeric]], target_values: &[Numeric]) -> Self {
        let mut value_count = HashMap::new();
        
        for &value in target_values {
            *value_count.entry(value as i64).or_insert(0) += 1;
        }
        
        Self {
            value_to_replace: value_count.iter().max_by_key(|&(_, count)| count).map(|(val, _)| val).expect("Mode not found!")
        }
    }
    
    fn predict(&self, sample: &[Numeric]) -> Result<Numeric, Box<dyn Error>> {
        Ok(self.value_to_replace)
    }
}