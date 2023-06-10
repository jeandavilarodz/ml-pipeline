use crate::data::data_frame::DataFrame;
use crate::types::Numeric;

use std::error::Error;

use rand::seq::SliceRandom;
use rand::thread_rng;

use itertools::Itertools;

pub struct KFold;

impl KFold {
    pub fn partition(
        table: &DataFrame<Numeric>,
        target_idx: usize,
        k: usize,
    ) -> Result<Vec<(Vec<usize>, Vec<usize>)>, Box<dyn Error>> {
        if table.get_column_idx(target_idx).is_none() {
            return Err("Couldn't find index of column of target value!".into());
        }

        let target_values = table.get_column_idx(target_idx).unwrap();
        let num_samples = target_values.values().len();
        let fold_size = num_samples / k;

        // Shuffle indexes
        let mut class_indices = (0..num_samples).collect_vec();
        class_indices.shuffle(&mut thread_rng());

        // Generate indeces for k - 1 folds
        let mut ret = Vec::with_capacity(k);
        for idx in 0..k - 1 {
            println!("FOLD: {}", idx + 1);
            let start = idx * fold_size;
            let end = (idx + 1) * fold_size;
            ret.push((
                [&class_indices[0..start], &class_indices[end..]].concat(),
                class_indices[start..end].to_vec(),
            ));
        }

        // Generate indeces for fold k, making the fold size a bit larger using a remainder
        let fold_size = (num_samples / k) + (num_samples % k);
        let start = num_samples - fold_size;
        ret.push((
            class_indices[0..start].to_vec(),
            class_indices[start..].to_vec(),
        ));

        Ok(ret)
    }
}
