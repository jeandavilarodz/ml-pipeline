use crate::data::data_frame::DataFrame;
use crate::types::Numeric;
use super::Partitioner;

use std::collections::HashMap;
use std::error::Error;

use rand::seq::SliceRandom;
use rand::thread_rng;

use num_traits::ToPrimitive;
use itertools::Itertools;

pub struct KFold;

impl Partitioner for KFold {
    fn partition(
        table: &DataFrame<Numeric>,
        parameters: HashMap<String, Numeric>,
    ) -> Result<Vec<(Vec<usize>, Vec<usize>)>, Box<dyn Error>> {
        let label_column_idx = parameters
            .get("index")
            .ok_or("index parameter not present!")?
            .to_usize()
            .ok_or("Could not parse index as usize!")?;

        if table.get_column_idx(label_column_idx).is_none() {
            return Err("Couldn't find index of column of target value!".into());
        }

        let k = parameters
            .get("num_folds")
            .ok_or("num_folds parameter not present!")?
            .to_usize()
            .ok_or("Could not parse num_folds as usize!")?;

        let label_column = table.get_column_idx(label_column_idx).unwrap();
        let num_samples = label_column.values().len();
        let fold_size = num_samples / k;

        // Shuffle indexes
        let mut indexes = (0..num_samples).collect_vec();
        indexes.shuffle(&mut thread_rng());

        // Generate indeces for k - 1 folds
        let mut ret = Vec::with_capacity(k);
        for idx in 0..k - 1 {
            let start = idx * fold_size;
            let end = (idx + 1) * fold_size;
            ret.push((
                [&indexes[0..start], &indexes[end..]].concat(),
                indexes[start..end].to_vec(),
            ));
        }

        // Generate indeces for last fold. Also making the fold use the remainder values.
        let fold_size = (num_samples / k) + (num_samples % k);
        let start = num_samples - fold_size;
        ret.push((indexes[0..start].to_vec(), indexes[start..].to_vec()));

        Ok(ret)
    }
}
