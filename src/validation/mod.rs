//! This contains the abstractions for all the ML models

mod classification_score;
mod mse;

use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

use lazy_static::lazy_static;

pub trait Evaluator {
    fn evaluate(&self, predictions: &[Numeric], target_values: &[Numeric]) -> Result<Numeric, Box<dyn Error>>;
}

lazy_static! {
    static ref EVALUATION_REPOSITORY: HashMap<&'static str, Box<dyn Evaluator + Sync>> =
        HashMap::from([
            (
                "MSE",
                Box::new(mse::MeanSquaredErrorEvaluator) as Box<dyn Evaluator + Sync>
            ),
            (
                "classification-score",
                Box::new(classification_score::ClassificationScoreEvaluator) as Box<dyn Evaluator + Sync>
            )
        ]);
}

pub fn evaluate_model(
    evaluation_name: &str,
    predictions: &[Numeric],
    target_values: &[Numeric],
) -> Result<Numeric, Box<dyn Error>> {
    if !EVALUATION_REPOSITORY.contains_key(evaluation_name) {
        return Err("Evaluation strategy not supported!".into());
    }
    EVALUATION_REPOSITORY[evaluation_name].evaluate(predictions, target_values)
}
