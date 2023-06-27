// knn_edited.rs

//! This file implements the logic to train an edited k-nearest neighbor learner

use super::Model;
use super::ModelBuilder;

use crate::models::knn::KNearestNeighbor;
use crate::types::{Numeric, NUMERIC_DIGIT_PRECISION};

use std::collections::HashMap;
use std::error::Error;

pub struct EditedKNearestNeighborTrainer {
    hyperparameters: Option<HashMap<String, String>>,
    num_neighbors: usize,
    epsilon: f64,
    show_voronoi: bool,
}

impl ModelBuilder for EditedKNearestNeighborTrainer {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            num_neighbors: 1,
            epsilon: NUMERIC_DIGIT_PRECISION,
            hyperparameters: None,
            show_voronoi: true,
        }
    }

    fn with_hyperparameters(
        &mut self,
        features: &HashMap<String, String>,
    ) -> Result<(), Box<dyn Error>> {
        self.hyperparameters = Some(features.clone());
        Ok(())
    }

    fn build(
        &mut self,
        training_values: &[Box<[Numeric]>],
        target_value_idx: usize,
    ) -> Result<Box<dyn Model>, Box<dyn Error>> {
        if training_values.is_empty() {
            return Err("Empty training set given!".into());
        }
        if target_value_idx >= training_values[0].len() {
            return Err("Target value index is out of bounds!".into());
        }

        // Build examples for the algorithm
        let mut label_examples = Vec::new();

        // Copy all the input training values as label examples
        label_examples.extend(training_values.iter().cloned());

        // Calculate training value mean
        let training_value_mean = training_values.iter().fold(0.0, |acc, x| {
            acc + x[target_value_idx]
        }) / training_values.len() as f64;

        // Calculate training value variance
        let training_value_variance = training_values.iter().fold(0.0, |acc, sample| {
            acc + (sample[target_value_idx] - training_value_mean).powi(2)
        }) / (training_values.len() - 1) as f64;

        // Generate model using internal parameters
        let mut model = KNearestNeighbor {
            num_neighbors: self.num_neighbors,
            label_index: target_value_idx,
            label_examples,
            epsilon: self.epsilon,
            gamma: training_value_variance.sqrt(),
        };

        if let Some(hyperparameters) = self.hyperparameters.as_ref() {
            model.set_hyperparameters(hyperparameters)?;
        }

        // Predict values and if the label doesn't match add the input value to the set
        for (idx, sample) in training_values.iter().enumerate().rev() {
            // Remove current sample from list of label examples
            model.label_examples.remove(idx);

            // Predict value of current sample with the rest of the data set
            let prediction = model.predict(sample);

            if (prediction - sample[model.label_index]).abs() > self.epsilon {
                // Sample was predicted incorrectly, therefore the sample is essential to the set
                // and we must add it back to the set
                model.label_examples.push(sample.clone());
            }
            else {
                println!("Sample {} was removed!", idx);
            }
        }

        if self.show_voronoi {
            model.generate_voronoi_diagram()?;
        }

        Ok(Box::new(model))
    }
}
