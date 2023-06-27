//! This module contains the implemented ML models

mod knn;
mod knn_condensed;
mod knn_edited;
mod knn_simple;
mod null;

use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

pub trait Model {
    fn predict(&self, sample: &[Numeric]) -> Numeric;
    fn label(&self, sample: &[Numeric]) -> Numeric;
    fn type_id(&self) -> &'static str;
    fn get_hyperparameters(&self) -> HashMap<String, String>;
    fn set_hyperparameters(&mut self, hyperparameters: &HashMap<String, String>) -> Result<(), Box<dyn Error>>;
}

pub trait ModelBuilder {
    fn new() -> Self
    where
        Self: Sized;
    fn with_hyperparameters(&mut self, features: &HashMap<String, String>) -> Result<(), Box<dyn Error>>;
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
        "knn-simple" => Ok(Box::new(knn_simple::SimpleKNearestNeighborTrainer::new())),
        "knn-condensed" => Ok(Box::new(
            knn_condensed::CondensedKNearestNeighborTrainer::new(),
        )),
        "knn-edited" => Ok(Box::new(knn_edited::EditedKNearestNeighborTrainer::new())),
        _ => Err("Unsupported model: {model_name}".into()),
    }
}
