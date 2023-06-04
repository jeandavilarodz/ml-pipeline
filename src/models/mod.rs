//! This contains the abstractions for all the ML models

mod null;

use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

use lazy_static::lazy_static;

pub trait Model {
    fn predict(&self, sample: &[Numeric]) -> Result<Numeric, Box<dyn Error>>;
}

pub trait ModelFactory {
    fn from_training(
        &self,
        training_values: &[&[Numeric]],
        target_values: &[Numeric],
    ) -> Result<Box<dyn Model>, Box<dyn Error>>;
    // FUTURE WORK: Maybe a add a method to generate a model from a description of hyperparameters
}

lazy_static! {
    static ref MODEL_REPOSITORY: HashMap<&'static str, Box<dyn ModelFactory + Sync>> =
        HashMap::from([(
            "null_classifier",
            Box::new(null::NullModelFactory) as Box<dyn ModelFactory + Sync>
        ),
        (
            "null_regression",
            Box::new(null::NullRegressionModelFactory) as Box<dyn ModelFactory + Sync>
        )]);
}

pub fn from_training(
    model_name: &str,
    training_features: &[&[Numeric]],
    target_values: &[Numeric],
) -> Result<Box<dyn Model>, Box<dyn Error>> {
    if !MODEL_REPOSITORY.contains_key(model_name) {
        return Err("Model not supported!".into());
    }
    Ok(MODEL_REPOSITORY[model_name].from_training(training_features, target_values)?)
}
