use num_traits::ToPrimitive;

use super::Evaluator;
use crate::types::Numeric;

use std::error::Error;

pub struct MeanSquaredErrorEvaluator;

impl Evaluator for MeanSquaredErrorEvaluator {
    fn evaluate(
        predictions: &Vec<Numeric>,
        training_samples: &Vec<Vec<&Numeric>>,
        training_label_idx: usize,
    ) -> Result<f64, Box<dyn Error>> {
        if predictions.len() != training_samples.len() {
            return Err("Predictions and targets are not of the same size!".into());
        }

        let mse = predictions
            .iter()
            .zip(training_samples.iter())
            .fold(0.0, |acc, (&pred, tar)| {
                acc + (tar[training_label_idx] - pred) * (tar[training_label_idx] - pred)
            })
            / training_samples.len() as f64;
        Ok(mse.to_f64().ok_or("Could not transform numeric to f64")?)
    }
}
