//! This module contains the implemented ML models

mod null_classification;
mod null_regression;
mod knn_condensed;

use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

pub trait Model {
    fn from_parameters(parameters: &Option<HashMap<String, Numeric>>) -> Result<Self, Box<dyn Error>> where Self: Sized;
    fn predict(&self, sample: Box<[Numeric]>) -> Result<Numeric, Box<dyn Error>>;
    fn train(
        &mut self,
        training_values: &Vec<Box<[Numeric]>>,
        target_value_idx: usize,
    ) -> Result<(), Box<dyn Error>>;
}

pub fn get_model(model_name: &str, parameters: &Option<HashMap<String, Numeric>>) -> Result<Box<dyn Model>, Box<dyn Error>> {
    match model_name {
        "null-regression" => Ok(Box::new(null_regression::NullRegressionModel::from_parameters(parameters)?)),
        "null-classifier" => Ok(Box::new(null_classification::NullClassificationModel::from_parameters(parameters)?)),
        "knn-condensed" => Ok(Box::new(knn_condensed::CondensedKNN::from_parameters(parameters)?)),
        _ => Err("Unsupported model: {model_name}".into()),
    }
}
