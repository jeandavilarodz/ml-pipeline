//! This contains the abstractions for all the ML models

mod null;

use crate::types::Numeric;

use std::error::Error;

pub trait Model {
    fn predict(&self, sample: &[Numeric]) -> Result<Numeric, Box<dyn Error>>;
}

pub trait ModelFactory {
    fn build(
        &self,
        training_values: &[&[Numeric]],
        target_values: &[Numeric],
    ) -> Result<Box<dyn Model>, Box<dyn Error>>;
    // FUTURE WORK: Maybe a add a method to generate a model from a description of hyperparameters
}

pub fn get_model_builder(model_name: &str) -> Result<Box<dyn ModelFactory>, Box<dyn Error>> {
    match model_name {
        "null-regression" => Ok(Box::new(null::NullRegressionModelFactory)),
        "null-classifier" => Ok(Box::new(null::NullModelFactory)),
        _ => Err("Unsupported model: {model_name}".into()),
    }
}
