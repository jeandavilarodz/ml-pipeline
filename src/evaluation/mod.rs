//! This contains the abstractions for all the ML models

mod classification_score;
mod mse;

use crate::types::Numeric;

use std::error::Error;

pub trait Evaluator {
    fn evaluate(
        predictions: &[Numeric],
        target_values: &[Numeric],
    ) -> Result<f64, Box<dyn Error>>;
}

type EvaluationFnPtr = fn(&[Numeric], &[Numeric]) -> Result<f64, Box<dyn Error>>;

pub fn get_evaluator(name: &str) -> Result<EvaluationFnPtr, Box<dyn Error>> {
    match name {
        "classification-score" => Ok(classification_score::ClassificationScoreEvaluator::evaluate),
        "mse" => Ok(mse::MeanSquaredErrorEvaluator::evaluate),
        _ => Err("Evaluation strategy not supported: {name}".into())
    }

}
