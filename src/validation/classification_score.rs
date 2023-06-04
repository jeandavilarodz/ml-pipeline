use super::Evaluator;
use crate::types::Numeric;

use std::error::Error;

pub struct ClassificationScoreEvaluator;

impl Evaluator for ClassificationScoreEvaluator {
    fn evaluate(
        &self,
        predictions: &[Numeric],
        target_values: &[Numeric],
    ) -> Result<Numeric, Box<dyn Error>> {
        if predictions.len() != target_values.len() {
            return Err("Predictions and targets are not of the same size!".into());
        }

        let correct = predictions
            .iter()
            .zip(target_values.iter())
            .fold(0, |acc, (&pred, &tar)| {
                if (pred as i64) != (tar as i64) {
                    acc + 1
                } else {
                    acc
                }
            });

        Ok(correct as Numeric / target_values.len() as Numeric)
    }
}
