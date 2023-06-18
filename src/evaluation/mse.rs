// mse.rs

/// This evaluator implements the logic for the mean-squared-error evaluation strategy.
use num_traits::ToPrimitive;

use super::Evaluator;
use crate::types::Numeric;

use std::error::Error;

pub struct MeanSquaredErrorEvaluator;

impl Evaluator for MeanSquaredErrorEvaluator {
    fn evaluate(
        predictions: &Vec<Numeric>,
        training_samples: &Vec<Box<[Numeric]>>,
        training_label_idx: usize,
    ) -> Result<f64, Box<dyn Error>> {
        if predictions.len() != training_samples.len() {
            return Err("Predictions and targets are not of the same size!".into());
        }

        // Iterate over all the target and training labels and accumulate the MSE metric
        let mut mse = predictions.iter().zip(training_samples.iter()).fold(
            0.0,
            |acc, (&prediction, target)| {
                acc + ((target[training_label_idx] - prediction)
                    * (target[training_label_idx] - prediction))
            },
        );

        // Divide the accumulated MSE metric over the number of samples
        mse /= training_samples.len() as f64;
        Ok(mse.to_f64().ok_or("Could not transform numeric to f64")?)
    }
}
