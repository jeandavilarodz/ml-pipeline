// knn_condensed.rs

//! This file implements the logic to train a condensed k-nearest neighbor learner

use super::Model;
use super::ModelBuilder;

use crate::models::knn_classifier::KNearestNeighbor;
use crate::types::{Numeric, NUMERIC_DIGIT_PRECISION};

use std::collections::HashMap;
use std::error::Error;

pub struct CondensedKNearestNeighborTrainer {
    features: Option<Vec<Box<[Numeric]>>>,
    num_neighbors: usize,
    epsilon: f64,
    show_voronoi: bool,
}

impl ModelBuilder for CondensedKNearestNeighborTrainer {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            features: None,
            num_neighbors: 1,
            epsilon: NUMERIC_DIGIT_PRECISION,
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

        // Build iterator over training values
        let mut training_data = training_values.iter();

        // Build examples for the algorithm
        let mut label_examples = Vec::new();

        // If there are features, use them
        if let Some(features) = self.features.as_ref() {
            label_examples.extend(features.iter().cloned());
        }

        // If the label examples are empty then add first sample as a label example
        if label_examples.is_empty() {
            label_examples.push(training_data.next().unwrap().clone());
        }

        // Build k-nearest neighbors model with the label examples
        let mut model = KNearestNeighbor {
            num_neighbors: self.num_neighbors,
            label_index: target_value_idx,
            label_examples,
        };

        // Predict values and if the label doesn't match add the input value to the set
        for sample in training_data {
            let prediction = model.predict(sample);
            if (prediction - sample[model.label_index]).abs() > self.epsilon {
                model.label_examples.push(sample.clone());
            }
        }

        if self.show_voronoi {
            model.generate_voronoi_diagram()?;
        }

        Ok(Box::new(model))
    }
}
