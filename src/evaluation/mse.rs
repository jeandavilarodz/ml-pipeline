use num_traits::ToPrimitive;

use super::Evaluator;
use crate::types::Numeric;

use std::error::Error;

pub struct MeanSquaredErrorEvaluator;

impl Evaluator for MeanSquaredErrorEvaluator {
    fn evaluate(
        &self,
        predictions: &[Numeric],
        target_values: &[Numeric],
    ) -> Result<f64, Box<dyn Error>> {
        if predictions.len() != target_values.len() {
            return Err("Predictions and targets are not of the same size!".into());
        }

        let mse = predictions
            .iter()
            .zip(target_values.iter())
            .fold(Numeric::from(0.0), |acc, (&pred, &tar)| acc + (tar - pred) * (tar - pred))
            / target_values.len() as f64;
        Ok(mse.to_f64().ok_or("Could not transform numeric to f64")?)
    }
}
