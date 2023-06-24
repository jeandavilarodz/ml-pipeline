//! This module contains the implemented ML models

mod knn_classifier;
mod knn_condensed;
mod knn_edited;
mod null;

use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

pub trait Model {
    fn predict(&self, sample: &[Numeric]) -> Numeric;
}

pub trait ModelBuilder {
    fn new() -> Self
    where
        Self: Sized;
    fn with_parameters(
        &mut self,
        parameters: &Option<HashMap<String, Numeric>>,
    ) -> Result<(), Box<dyn Error>>;
    fn with_features(&mut self, features: &[Box<[Numeric]>]) -> Result<(), Box<dyn Error>>;
    fn build(
        &mut self,
        training_values: &[Box<[Numeric]>],
        target_value_idx: usize,
    ) -> Result<Box<dyn Model>, Box<dyn Error>>;
}

pub fn get_model_builder(model_name: &str) -> Result<Box<dyn ModelBuilder>, Box<dyn Error>> {
    match model_name {
        "null-regression" => Ok(Box::new(null::NullRegressionModelTrainer::new())),
        "null-classifier" => Ok(Box::new(null::NullClassificationModelTrainer::new())),
        "knn-condensed" => Ok(Box::new(
            knn_condensed::CondensedKNearestNeighborTrainer::new(),
        )),
        "knn-edited" => Ok(Box::new(knn_edited::EditedKNearestNeighborTrainer::new())),
        _ => Err("Unsupported model: {model_name}".into()),
    }
}
