use crate::data::data_frame::DataFrame;
use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

use rand::seq::SliceRandom;
use rand::thread_rng;

use itertools::Itertools;
use itertools::izip;

pub struct KFold;

impl KFold {
    pub fn partition(
        table: &DataFrame<Numeric>,
        target_idx: usize,
        k: usize,
    ) -> Result<(), Box<dyn Error>> {
        if table.get_column_idx(target_idx).is_none() {
            return Err("Couldn't find index of column of target value!".into());
        }

        let target_values = table.get_column_idx(target_idx).unwrap();
        let num_samples = target_values.values().len();
        let fold_size = num_samples / k;

        // Shuffle indexes
        let mut class_indices = vec![0..num_samples];
        class_indices.shuffle(&mut thread_rng);

        let mut validation_set: Vec<Vec<usize>> = Vec::with_capacity(k);
        let mut training_set: Vec<Vec<usize>> = Vec::with_capacity(k);

        for idx in 0..k-1 {
            println!("FOLD: {}", idx + 1);
            let start = idx * fold_size;
            let end = (idx + 1) * fold_size;
            validation_set.push(class_indices[start..end].to_vec());
            training_set.push([&class_indices[0..start], &class_indices[end..]].concat());
        }

        let fold_size = (num_samples / k) + (num_samples % k);
        let start = num_samples - fold_size;
        validation_set.push(class_indices[start..].to_vec());
        training_set.push(class_indices[0..start].to_vec());

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
