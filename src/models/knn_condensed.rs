// knn_condensed.rs

//! This file implements the logic to train a condensed k-nearest neighbor learner

use super::Model;
use super::ModelTrainer;

use crate::models::knn_classifier::KNearestNeighbor;
use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

use num_traits::ToPrimitive;

pub struct CondensedKNearestNeighborTrainer {
    training_data: Option<Vec<Box<[Numeric]>>>,
    num_neighbors: Option<usize>,
    label_index: Option<usize>,
    model_snapshot: Vec<Box<[Numeric]>>,
}

impl ModelTrainer for CondensedKNearestNeighborTrainer {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            num_neighbors: None,
            training_data: None,
            label_index: None,
            model_snapshot: vec![],
        }
    }

    fn with_parameters(
        &mut self,
        parameters: &Option<HashMap<String, Numeric>>,
    ) -> Result<(), Box<dyn Error>> {
        let parameters = parameters.as_ref().ok_or("No parameters given!")?;
        let num_neighbors = parameters
            .get("num_neighbors")
            .ok_or("num_neighbors parameter not present!")?
            .to_usize()
            .ok_or("Could not parse num_neighbors as usize!")?;
        self.num_neighbors = Some(num_neighbors);
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
        let mut model = KNearestNeighbor {
            num_neighbors: self.num_neighbors.ok_or("no num_neighbors")?,
            label_index: self.label_index.ok_or("no label_index")?,
            label_examples: self.model_snapshot.clone(),
        };

        // Predict values and if the label doesn't match add the input value to the set
        for sample in self
            .training_data
            .as_ref()
            .ok_or("No training data!")?
            .into_iter()
        {
            let prediction = model.predict(sample.clone());
            if (prediction - sample[model.label_index]).abs() < 1e-8 {
                self.model_snapshot.push(sample.clone());
            }
        }

        model.label_examples = self.model_snapshot.clone();

        Ok(Box::new(model))
    }
}