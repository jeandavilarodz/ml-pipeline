// knn_condensed.rs

//! This file implements the logic to train a condensed k-nearest neighbor learner

use super::Model;
use super::ModelTrainer;

use crate::models::knn_classifier::KNearestNeighbor;
use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

pub struct CondensedKNearestNeighborTrainer {
    training_data: Option<Vec<Box<[Numeric]>>>,
    num_neighbors: Option<usize>,
    label_index: Option<usize>,
    epsilon: f64,
    model_snapshot: Vec<Box<[Numeric]>>,
}

impl ModelTrainer for CondensedKNearestNeighborTrainer {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            num_neighbors: Some(1),
            training_data: None,
            label_index: None,
            epsilon: 1e-8,
            model_snapshot: vec![],
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

    fn with_training_data(
        &mut self,
        training_values: &Vec<Box<[Numeric]>>,
        label_idx: usize,
    ) -> Result<(), Box<dyn Error>> {
        if training_values.len() < 1 {
            return Err("Empty training set given!".into());
        }
        if training_values.get(label_idx).is_none() {
            return Err("Could not find target label in training data!".into());
        }
        self.training_data = Some(training_values.clone());
        self.label_index = Some(label_idx);
        Ok(())
    }

    fn train(&mut self) -> Result<Box<dyn Model>, Box<dyn Error>> {
        let training_data = self.training_data.as_mut().ok_or("No training data!")?;

        if self.model_snapshot.is_empty() {
            self.model_snapshot
                .push(training_data.get(0).unwrap().clone());
            println!("First model snapshot: {:?}", self.model_snapshot);
        }

        let mut model = KNearestNeighbor {
            num_neighbors: self.num_neighbors.ok_or("no num_neighbors")?,
            label_index: self.label_index.ok_or("no label_index")?,
            label_examples: self.model_snapshot.clone(),
        };

        // Predict values and if the label doesn't match add the input value to the set
        let mut new_samples = Vec::new();
        for sample in training_data.iter() {
            let prediction = model.predict(sample.to_owned());
            if (prediction - sample[model.label_index]).abs() > self.epsilon {
                model.label_examples.push(sample.clone());
                new_samples.push(sample.clone());
            }
        }

        self.model_snapshot.extend(new_samples.iter().cloned());
        println!("Model snapshot: {:?}", self.model_snapshot);

        model.generate_voronoi_diagram()?;

        Ok(Box::new(model))
    }
}
