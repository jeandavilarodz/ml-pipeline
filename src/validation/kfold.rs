// kfold.rs

/// This file contains the logic to generate indexes corresponding to K-fold cross-validation.
/// Uses the defined data frame and index of label columns to generate k list of indexes which
/// the training and validation algorithm can use to index into the original data structure to
/// generate sections of training and validation data to train a model.

use crate::data::data_frame::DataFrame;
use crate::types::Numeric;
use super::Partitioner;

use std::collections::HashMap;
use std::error::Error;

use rand::seq::SliceRandom;

pub struct KFold;

impl Partitioner for KFold {
    fn partition(
        table: &DataFrame<Numeric>,
        label_column_idx: usize,
        parameters: &HashMap<String, Numeric>,
    ) -> Result<Vec<(Vec<usize>, Vec<usize>)>, Box<dyn Error>> {
        // Making sure the input is valid and correct
        if table.get_column_idx(label_column_idx).is_none() {
            return Err("Couldn't find index of column of target value!".into());
        }

        let k = *parameters
            .get("num_folds")
            .ok_or("num_folds parameter not present!")? as usize;

        let label_column = table.get_column_idx(label_column_idx).unwrap();
        let num_samples = label_column.values().len();
        let fold_size = num_samples / k;

        // Shuffle indexes
        let mut indexes = (0..num_samples).collect::<Vec<usize>>();
        indexes.shuffle(&mut rand::thread_rng());

        // Generate indeces for k - 1 folds
        let mut ret = Vec::with_capacity(k);
        for idx in 0..k - 1 {
            // Calculate the start and end value for the array
            let start = idx * fold_size;
            let end = (idx + 1) * fold_size;

            // Push indexes to the output
            ret.push((
                [&indexes[0..start], &indexes[end..]].concat(),
                indexes[start..end].to_vec(),
            ));
        }

        // Generate indeces for last fold. Also making the fold use the remainder values.
        let fold_size = (num_samples / k) + (num_samples % (num_samples / k));
        let start = num_samples - fold_size;
        ret.push((indexes[0..start].to_vec(), indexes[start..].to_vec()));

        Ok(ret)
    }
}
