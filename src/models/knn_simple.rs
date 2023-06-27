// knn_condensed.rs

//! This file implements the logic to train a condensed k-nearest neighbor learner

use super::Model;
use super::ModelBuilder;

use crate::models::knn::KNearestNeighbor;
use crate::types::{Numeric, NUMERIC_DIGIT_PRECISION};

use std::collections::HashMap;
use std::error::Error;

pub struct SimpleKNearestNeighborTrainer {
    hyperparameters: Option<HashMap<String, String>>,
    num_neighbors: usize,
    epsilon: f64,
    show_voronoi: bool,
}

impl ModelBuilder for SimpleKNearestNeighborTrainer {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            hyperparameters: None,
            num_neighbors: 1,
            epsilon: NUMERIC_DIGIT_PRECISION,
            show_voronoi: false,
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
        
        // Push all training values to be label examples
        label_examples.extend(training_values.iter().cloned());

        // Build k-nearest neighbors model with the label examples
        let mut model = KNearestNeighbor {
            num_neighbors: self.num_neighbors,
            label_index: target_value_idx,
            label_examples,
            epsilon: self.epsilon,
            gamma: 1.0,
        };

        if let Some(hyperparameters) = self.hyperparameters.as_ref() {
            model.set_hyperparameters(hyperparameters)?;
        }

        if self.show_voronoi {
            model.generate_voronoi_diagram()?;
        }

        Ok(Box::new(model))
    }
}
