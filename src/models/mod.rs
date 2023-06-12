//! This contains the abstractions for all the ML models

mod null_classification;
mod null_regression;

use crate::types::Numeric;

use std::error::Error;

pub trait Model {
    fn new() -> Self where Self: Sized;
    fn predict(&self, sample: &Vec<&Numeric>) -> Result<Numeric, Box<dyn Error>>;
    fn train(
        &mut self,
        training_values: &Vec<Vec<&Numeric>>,
        target_value_idx: usize,
    ) -> Result<(), Box<dyn Error>>;
}

pub fn get_model(model_name: &str) -> Result<Box<dyn Model>, Box<dyn Error>> {
    match model_name {
        "null-regression" => Ok(Box::new(null_regression::NullRegressionModel::new())),
        "null-classifier" => Ok(Box::new(null_classification::NullClassificationModel::new())),
        _ => Err("Unsupported model: {model_name}".into()),
    }
}
