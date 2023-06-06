use super::Evaluator;
use crate::types::Numeric;

use std::error::Error;

pub struct MeanSquaredErrorEvaluator;

impl Evaluator for MeanSquaredErrorEvaluator {
    fn evaluate(
        &self,
        predictions: &[Numeric],
        target_values: &[Numeric],
    ) -> Result<Numeric, Box<dyn Error>> {
        if predictions.len() != target_values.len() {
            return Err("Predictions and targets are not of the same size!".into());
        }

        let mse = predictions
            .iter()
            .zip(target_values.iter())
            .fold(0.0, |acc, (&pred, &tar)| acc + (tar - pred) * (tar - pred))
            / target_values.len() as Numeric;
        Ok(mse)
    }
}
