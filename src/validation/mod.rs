//! This module contains the logic responsible for creating indexes into the original data structure
//! to perform a cross-validation to evaluate a model

mod kfold_stratified;
mod kfold;

use std::collections::HashMap;
use std::error::Error;

use crate::data::data_frame::DataFrame;
use crate::types::Numeric;

pub trait Partitioner {
    fn partition(
        table: &DataFrame<Numeric>,
        label_column_idx: usize,
        parameters: &HashMap<String, Numeric>,
    ) -> Result<Vec<(Vec<usize>, Vec<usize>)>, Box<dyn Error>>;
}

type PartitionerFnPtr = fn(&DataFrame<Numeric>, usize, &HashMap<String, Numeric>) -> Result<Vec<(Vec<usize>, Vec<usize>)>, Box<dyn Error>>;

pub fn get_partitioner(partitioner: &str) -> Result<PartitionerFnPtr, Box<dyn Error>> {
    match partitioner {
        "stratified-kfold" => Ok(kfold_stratified::StratifiedKFold::partition),
        "kfold" => Ok(kfold::KFold::partition),
        _ => Err("Partitioner not supported: {partitioner}".into()),
    }
}
