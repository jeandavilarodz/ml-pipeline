// knn_edited.rs

//! This file implements the logic to train an edited k-nearest neighbor learner

use super::Model;
use super::ModelBuilder;

use crate::models::knn_classifier::KNearestNeighbor;
use crate::types::{Numeric, NUMERIC_DIGIT_PRECISION};

use std::collections::HashMap;
use std::error::Error;

pub struct EditedKNearestNeighborTrainer {
    features: Option<Vec<Box<[Numeric]>>>,
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
            features: None,
            show_voronoi: true,
        }
    }

    fn with_parameters(
        &mut self,
        parameters: &Option<HashMap<String, Numeric>>,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(parameters) = parameters {
            if let Some(epsilon) = parameters.get("epsilon") {
                self.epsilon = *epsilon;
                println!("Set epsilon to {}", self.epsilon);
            }
        }
        Ok(())
    }

    fn with_features(
        &mut self,
        features: &HashMap<String, String>,
    ) -> Result<(), Box<dyn Error>> {
        let mut label_examples: Vec<Box<[Numeric]>> = Vec::new();
        for (key, val) in features.iter() {
            match key.as_str() {
                "label_index" => {},
                "num_neighbors" => {
                    self.num_neighbors = val.parse::<usize>()?;
                }
                _ => {
                    label_examples.push(
                        val.split(",").filter_map(|v| {
                            v.trim().parse::<Numeric>().ok()
                        }).collect()
                    );
                }
            }
        }
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

        // If there are features given use them as label examples
        let mut offset = 0;
        if let Some(features) = &self.features {
            label_examples.extend(features.iter().cloned());
            offset += features.len();
        }

        // Copy all the input training values as label examples
        label_examples.extend(training_values.iter().cloned());

        // Generate model using internal parameters
        let mut model = KNearestNeighbor {
            num_neighbors: self.num_neighbors,
            label_index: target_value_idx,
            label_examples,
        };

        // Predict values and if the label doesn't match add the input value to the set
        for (idx, sample) in training_values.iter().enumerate().rev() {
            // Remove current sample from list of label examples
            model.label_examples.remove(idx + offset);

            // Predict value of current sample with the rest of the data set
            let prediction = model.predict(sample);

            if (prediction - sample[model.label_index]).abs() > self.epsilon {
                // Sample was predicted incorrectly, therefore the sample is essential to the set
                // and we must add it back to the set
                model.label_examples.push(sample.clone());
            }
        }

        if self.show_voronoi {
            model.generate_voronoi_diagram()?;
        }

        Ok(Box::new(model))
    }
}
