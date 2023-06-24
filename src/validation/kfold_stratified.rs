// kfold_stratified.rs

use super::Partitioner;
/// This file contains the logic to generate indexes corresponding to a stratified K-fold cross-validation,
/// where original class label distributions are conserved. Uses the defined data frame and index of label
/// columns to generate k lists of indexes which the training and validation algorithm can use to index
/// into the original data structure and generate sections of training and validation data to train an model.
use crate::data::data_frame::DataFrame;
use crate::types::{Numeric, NUMERIC_DIGIT_PRECISION};

use std::collections::HashMap;
use std::error::Error;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct StratifiedKFold;

impl Partitioner for StratifiedKFold {
    fn partition(
        table: &DataFrame<Numeric>,
        label_column_idx: usize,
        parameters: &HashMap<String, Numeric>,
    ) -> Result<Vec<(Vec<usize>, Vec<usize>)>, Box<dyn Error>> {
        // Verify input is valid
        if table.get_column_idx(label_column_idx).is_none() {
            return Err("Couldn't find index of column of target value!".into());
        }

        let k = *parameters
            .get("num_folds")
            .ok_or("num_folds parameter not present!")? as usize;

        // Get a reference to the column that contains the class labels
        let label_column = table
            .get_column_idx(label_column_idx)
            .ok_or("Could not find column of target values!")?;

        // Populate a map of label values with a list of all the values that are within
        let mut label_indices = HashMap::new();
        label_column.values().enumerate().for_each(|(idx, value)| {
            let key = (value / NUMERIC_DIGIT_PRECISION) as i64;
            let index_list = label_indices.entry(key).or_insert(vec![]);
            index_list.push(idx);
        });

        // Shuffle the list of indexes for each class label
        label_indices
            .values_mut()
            .for_each(|indices| indices.shuffle(&mut thread_rng()));

        // Printing for debugging purposes
        for (label, index_list) in label_indices.iter() {
            println!(
                "label: {} | n: {} | p: {}",
                (*label as f64) * NUMERIC_DIGIT_PRECISION,
                index_list.len(),
                100.0 * index_list.len() as f32 / label_column.values().len() as f32
            );
        }

        let mut ret = Vec::with_capacity(k);

        // Create index list for K-1 folds, these should be equal sized
        for idx in 0..k - 1 {
            println!("FOLD: {}", idx + 1);
            let mut validation_indices = Vec::new();
            let mut training_indices = Vec::new();
            let mut label_sizes = Vec::new();

            // iterate through each class label in the map of class labels
            for indexes in label_indices.values() {
                // Calculate the number of items of that class we should pick
                // as well as the starting and ending indexes
                let fold_size = indexes.len() / k;
                let start = idx * fold_size;
                let end = (idx + 1) * fold_size;

                // Copy the indexes to a list of all indexes pertaining to training and validation
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

            // Push the pair of training and validation sets to the output
            ret.push((training_indices, validation_indices));
        }

        // Calculate values in the last fold
        let mut validation_indices = Vec::new();
        let mut training_indices = Vec::new();
        let mut label_sizes = Vec::new();
        println!("FOLD: {}", k);
        for indexes in label_indices.values() {
            // Calculate the fold size and extending by the residual value
            let mut fold_size = indexes.len() / k;
            fold_size += indexes.len() % fold_size;
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

        // Push the last fold to the output
        ret.push((training_indices, validation_indices));

        Ok(ret)
    }
}
