// classification_score.rs

/// This file contains the logic for the 0-1 evaluation strategy. Where the total amount of correctly predicted
/// labels are divided by the total number of labels.

use super::Evaluator;
use crate::types::Numeric;

use std::error::Error;

pub struct ClassificationScoreEvaluator;

impl Evaluator for ClassificationScoreEvaluator {
    fn evaluate(
        predictions: &Vec<Numeric>,
        training_samples: &Vec<Box<[Numeric]>>,
        training_label_idx: usize,
    ) -> Result<f64, Box<dyn Error>> {
        if predictions.len() != training_samples.len() {
            return Err("Predictions and targets are not of the same size!".into());
        }

        let num_samples = training_samples.len();

        let error_count = predictions
            .iter()
            .zip(training_samples.iter())
            .fold(0, |acc, (&pred, tar)| {
                // If class values are the same they should be close to zero
                if (pred - tar[training_label_idx]).abs() > 1e-8 {
                    acc + 1
                } else {
                    acc
                }
            });

        Ok(error_count as f64 / num_samples as f64)
    }
}
