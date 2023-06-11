use crate::data::data_frame::DataFrame;
use crate::types::Numeric;
use super::Partitioner;

use std::collections::HashMap;
use std::error::Error;

use num_traits::ToPrimitive;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct StratifiedKFold;

impl Partitioner for StratifiedKFold {
    fn partition(
        table: &DataFrame<Numeric>,
        label_column_idx: usize,
        parameters: HashMap<String, Numeric>,
    ) -> Result<Vec<(Vec<usize>, Vec<usize>)>, Box<dyn Error>> {
        if table.get_column_idx(label_column_idx).is_none() {
            return Err("Couldn't find index of column of target value!".into());
        }

        let k = parameters
            .get("num_folds")
            .ok_or("num_folds parameter not present!")?
            .to_usize()
            .ok_or("Could not parse num_folds as usize!")?;

        let label_column = table.get_column_idx(label_column_idx).unwrap();

        // Populate set of class entries with indexes
        let mut label_indices = HashMap::new();
        for (idx, &value) in label_column.values().enumerate() {
            let key = (value * 1e8)
                .to_i64()
                .ok_or("Could not turn Numeric into key!")?;
            let index_list = label_indices.entry(key).or_insert(vec![]);
            index_list.push(idx);
        }

        // Shuffle indexes in each vector
        label_indices
            .values_mut()
            .for_each(|indices| indices.shuffle(&mut thread_rng()));

        for (label, index_list) in label_indices.iter() {
            println!(
                "label: {} | n: {} | p: {}",
                label,
                index_list.len(),
                100.0 * index_list.len() as f32 / label_column.values().len() as f32
            );
        }

        let mut ret = Vec::with_capacity(k);

        for idx in 0..k - 1 {
            println!("FOLD: {}", idx + 1);
            let mut validation_indices = Vec::new();
            let mut training_indices = Vec::new();
            let mut label_sizes = Vec::new();
            for indexes in label_indices.values() {
                let fold_size = indexes.len() / k;
                let start = idx * fold_size;
                let end = (idx + 1) * fold_size;
                validation_indices.extend(indexes[start..end].to_vec());
                training_indices.extend([&indexes[0..start], &indexes[end..]].concat());
                label_sizes.push(indexes[0..start].len() + indexes[end..].len());
            }
            for (label, label_size) in label_sizes.into_iter().enumerate() {
                println!(
                    "label: {} | n: {} | p: {}",
                    label,
                    label_size,
                    100.0 * label_size as f32 / training_indices.len() as f32
                );
            }
            ret.push((training_indices, validation_indices));
        }

        let mut validation_indices = Vec::new();
        let mut training_indices = Vec::new();
        let mut label_sizes = Vec::new();
        println!("FOLD: {}", k);
        for indexes in label_indices.values() {
            let fold_size = (indexes.len() / k) + (indexes.len() % k);
            let end = indexes.len();
            let start = end - fold_size;
            validation_indices.extend(indexes[start..end].to_vec());
            training_indices.extend(indexes[0..start].to_vec());
            label_sizes.push(indexes[0..start].len());
        }
        for (label, label_size) in label_sizes.into_iter().enumerate() {
            println!(
                "label: {} | n: {} | p: {}",
                label,
                label_size,
                100.0 * label_size as f32 / training_indices.len() as f32
            );
        }
        ret.push((training_indices, validation_indices));

        Ok(ret)
    }
}
