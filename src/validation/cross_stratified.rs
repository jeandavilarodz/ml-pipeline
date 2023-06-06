use crate::data::data_frame::DataFrame;
use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

pub struct StratifiedCrossValidation;

impl StratifiedCrossValidation {
    pub fn partition(table: &DataFrame<Numeric>, target_idx: usize) -> Result<(), Box<dyn Error>> {
        let mut value_count = HashMap::new();

        if table.get_column_idx(target_idx).is_none() {
            return Err("Couldn't find index of column of target value!".into());
        }

        let target_values = table.get_column_idx(target_idx).unwrap();
        let num_targets = target_values.values().count();
    
        for &value in target_values.values() {
            let counter = value_count.entry(value as i64).or_insert(0);
            *counter += 1;
        }

        if let Some(metadata) = target_values.get_metadata() {
            println!("meta: {:#?}", metadata);
        }

        for (class_value, count) in value_count {
            let percent = count as f32 / num_targets as f32;
            println!("class: {class_value} | count: {count} | percent: {percent}");
        }

        Ok(())
    }
}