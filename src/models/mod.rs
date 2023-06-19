//! This module contains the implemented ML models

mod knn_classifier;
mod knn_condensed;
mod knn_edited;
mod null;

use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

pub trait Model {
    fn predict(&self, sample: Box<[Numeric]>) -> Numeric;
}

pub trait ModelTrainer {
    fn new() -> Self
    where
        Self: Sized;
    fn with_parameters(
        &mut self,
        parameters: &Option<HashMap<String, Numeric>>,
    ) -> Result<(), Box<dyn Error>>;
    fn with_training_data(
        &mut self,
        training_values: &Vec<Box<[Numeric]>>,
        target_value_idx: usize,
    ) -> Result<(), Box<dyn Error>>;
    fn train(&mut self) -> Result<Box<dyn Model>, Box<dyn Error>>;
}

pub fn get_model_builder(model_name: &str) -> Result<Box<dyn ModelTrainer>, Box<dyn Error>> {
    match model_name {
        "null-regression" => Ok(Box::new(null::NullRegressionModelTrainer::new())),
        "null-classifier" => Ok(Box::new(null::NullClassificationModelTrainer::new())),
        "knn-condensed" => Ok(Box::new(
            knn_condensed::CondensedKNearestNeighborTrainer::new(),
        )),
        _ => Err("Unsupported model: {model_name}".into()),
    }
}
