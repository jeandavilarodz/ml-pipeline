//! This module containes the evaluation strategies for a machine learning model

mod classification_score;
mod mse;

use crate::types::Numeric;

use std::error::Error;

pub trait Evaluator {
    fn evaluate(
        predictions: &[Numeric],
        training_samples: &[Box<[Numeric]>],
        training_label_idx: usize,
    ) -> Result<f64, Box<dyn Error>>;
}

type EvaluationFnPtr = fn(&[Numeric], &[Box<[Numeric]>], usize) -> Result<f64, Box<dyn Error>>;

pub fn get_evaluator(name: &str) -> Result<EvaluationFnPtr, Box<dyn Error>> {
    match name {
        "classification-score" => Ok(classification_score::ClassificationScoreEvaluator::evaluate),
        "mse" => Ok(mse::MeanSquaredErrorEvaluator::evaluate),
        _ => Err("Evaluation strategy not supported: {name}".into())
    }

}
