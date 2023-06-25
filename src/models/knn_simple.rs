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
    num_neighbors: Some<usize>,
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
            num_neighbors: None,
            epsilon: NUMERIC_DIGIT_PRECISION,
            show_voronoi: false,
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
            if let Some(num_neighbors) = parameters.get("num_neighbors") {
                self.num_neighbors = Some(*num_neighbors as usize);
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
                    self.num_neighbors = Some(val.parse::<usize>()?);
                }
                _ => {
                    label_examples.push(
                        val.split(',').filter_map(|v| {
                            v.trim().parse::<Numeric>().ok()
                        }).collect()
                    );
                }
            }
        }
        self.features = Some(label_examples);
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

        // If there are features, use them
        if let Some(features) = self.features.as_ref() {
            label_examples.extend(features.iter().cloned());
        }
        
        // Push all training values to be label examples
        label_examples.extend(training_values.iter().cloned());
        for sample in training_values.iter() {
            
        }

        // Build k-nearest neighbors model with the label examples
        let mut model = KNearestNeighbor {
            num_neighbors: self.num_neighbors.ok_or("No number of neighbors given!")?,
            label_index: target_value_idx,
            label_examples,
        };

        if self.show_voronoi {
            model.generate_voronoi_diagram()?;
        }

        Ok(Box::new(model))
    }
}
