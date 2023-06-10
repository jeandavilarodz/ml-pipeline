use crate::data::data_frame::DataFrame;
use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

use rand::seq::SliceRandom;
use rand::thread_rng;

use itertools::Itertools;

pub struct StratifiedCrossValidation;

impl StratifiedCrossValidation {
    pub fn partition(
        table: &DataFrame<Numeric>,
        target_idx: usize,
        k: usize,
    ) -> Result<(), Box<dyn Error>> {
        if table.get_column_idx(target_idx).is_none() {
            return Err("Couldn't find index of column of target value!".into());
        }

        let target_values = table.get_column_idx(target_idx).unwrap();

        // Populate set of class entries with indexes
        let mut class_indices: HashMap<i64, Vec<usize>> = HashMap::new();
        for (idx, &value) in target_values.values().enumerate() {
            let class_list = class_indices.entry(value as i64).or_insert(vec![]);
            class_list.push(idx);
        }

        // Shuffle indexes in each vector
        class_indices.values_mut().for_each(|indices| indices.shuffle(&mut thread_rng()));

        for (category, class_list) in class_indices.iter() {
            println!("cat: {} | n: {} | p: {}", category, class_list.len(), class_list.len() as f32 / target_values.values().len() as f32);
        }

        let mut validation_set: Vec<Vec<usize>> = Vec::with_capacity(k);
        let mut training_set: Vec<Vec<usize>> = Vec::with_capacity(k);

        for idx in 0..k-1 {
            let mut validation_indices = Vec::new();
            let mut training_indices = Vec::new();
            for (_key, indexes) in class_indices.iter() {
                let fold_size = indexes.len() / k;
                let start = idx * fold_size;
                let end = (idx + 1) * fold_size;
                validation_indices.extend(indexes[start..end].to_vec());
                training_indices.extend([&indexes[0..start], &indexes[end..]].concat());
            }
            validation_set.push(validation_indices);
            training_set.push(training_indices);
        }

        let mut validation_indices = Vec::new();
        let mut training_indices = Vec::new();
        for (_key, indexes) in class_indices.iter() {
            let fold_size = (indexes.len() / k) + (indexes.len() %  k);
            let end = indexes.len();
            let start = end - fold_size;
            validation_indices.extend(indexes[start..end].to_vec());
            training_indices.extend(indexes[0..start].to_vec());
        }
        validation_set.push(validation_indices);
        training_set.push(training_indices);

        for (train_indices, validation_indices) in training_set.iter().zip(validation_set.iter()) {
            let train_data: Vec<Vec<&Numeric>> = train_indices.iter().map(|&i| table.columns().filter_map(|col | col.get(i)).collect_vec()).collect();
            let validation_data: Vec<Vec<&Numeric>> = validation_indices.iter().map(|&i| table.columns().filter_map(|col | col.get(i)).collect_vec()).collect();
            println!("TRAINING");
            println!("{:?}", train_data);
            println!("TRAINING SIZE: {}", train_data.len());

            println!("VALIDATION");
            println!("{:?}", validation_data);
            println!("VALIDATION SIZE: {}", validation_data.len());
        }
                                                    
        Ok(())
    }
}
