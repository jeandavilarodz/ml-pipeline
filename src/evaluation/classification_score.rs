use super::Evaluator;
use crate::types::Numeric;

use std::error::Error;

pub struct ClassificationScoreEvaluator;

impl Evaluator for ClassificationScoreEvaluator {
    fn evaluate(
        predictions: &[Numeric],
        target_values: &[Numeric],
    ) -> Result<f64, Box<dyn Error>> {
        if predictions.len() != target_values.len() {
            return Err("Predictions and targets are not of the same size!".into());
        }

        let correct = predictions
            .iter()
            .zip(target_values.iter())
            .fold(0, |acc, (&pred, &tar)| {
                // If class values are the same they should be close to zero
                if (pred - tar).abs() < 1e-8 {
                    acc + 1
                } else {
                    acc
                }
            });

        Ok(correct as f64 / target_values.len() as f64)
    }
}
