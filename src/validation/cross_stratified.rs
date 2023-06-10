use crate::data::data_frame::DataFrame;
use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct StratifiedCrossValidation;

impl StratifiedCrossValidation {
    pub fn partition(
        table: &DataFrame<Numeric>,
        target_idx: usize,
        k: usize,
    ) -> Result<Vec<(Vec<usize>, Vec<usize>)>, Box<dyn Error>> {
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
            println!("cat: {} | n: {} | p: {}", category, class_list.len(), 100.0 * class_list.len() as f32 / target_values.values().len() as f32);
        }

        let mut ret = Vec::with_capacity(k);

        for idx in 0..k-1 {
            println!("FOLD: {}", idx + 1);
            let mut validation_indices = Vec::new();
            let mut training_indices = Vec::new();
            let mut class_sizes = Vec::new();
            for (_class_idx, indexes) in class_indices.iter() {
                let fold_size = indexes.len() / k;
                let start = idx * fold_size;
                let end = (idx + 1) * fold_size;
                validation_indices.extend(indexes[start..end].to_vec());
                training_indices.extend([&indexes[0..start], &indexes[end..]].concat());
                class_sizes.push(indexes[0..start].len() + indexes[end..].len());
            }
            for (class_idx, class_size) in class_sizes.into_iter().enumerate() {
                println!("cat: {} | n: {} | p: {}", class_idx, class_size, 100.0 * class_size as f32 / training_indices.len() as f32);
            }
            ret.push((training_indices, validation_indices));
        }

        let mut validation_indices = Vec::new();
        let mut training_indices = Vec::new();
        let mut class_sizes = Vec::new();
        println!("FOLD: {}", k);
        for (_class_idx, indexes) in class_indices.iter() {
            let fold_size = (indexes.len() / k) + (indexes.len() %  (indexes.len() / k));
            let end = indexes.len();
            let start = end - fold_size;
            validation_indices.extend(indexes[start..end].to_vec());
            training_indices.extend(indexes[0..start].to_vec());
            class_sizes.push(indexes[0..start].len());
        }
        for (class_idx, class_size) in class_sizes.into_iter().enumerate() {
            println!("cat: {} | n: {} | p: {}", class_idx, class_size, 100.0 * class_size as f32 / training_indices.len() as f32);
        }
        ret.push((training_indices, validation_indices));
                                                    
        Ok(ret)
    }
}
