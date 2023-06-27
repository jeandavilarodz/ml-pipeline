//! This module implements the logic for training algorithms using 
//! given partitioners, evaluators, and model builders.

mod simple;
mod kx2_folds;

use crate::data::data_frame::DataFrame;
use crate::config::ConfigStruct;
use crate::types::Numeric;

use std::error::Error;

pub trait TrainingDirector {
    fn train_and_evaluate(df: &DataFrame<Numeric>, configs: &ConfigStruct) -> Result<f64, Box<dyn Error>>;
}

pub fn train_and_evaluate(df: &DataFrame<Numeric>, configs: &ConfigStruct) -> Result<f64, Box<dyn Error>> {
    match configs.training.strategy.as_str() {
        "simple" => simple::train_and_evaluate(df, configs),
        "kx2-folds" => kx2_folds::train_and_evaluate(df, configs),
        _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Unknown strategy")))
    }
}
