// knn_edited.rs

//! This file implements the logic to train an edited k-nearest neighbor learner

use super::Model;
use super::ModelTrainer;

use crate::models::knn_classifier::KNearestNeighbor;
use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

pub struct EditedKNearestNeighborTrainer {
    training_data: Option<Vec<Box<[Numeric]>>>,
    num_neighbors: Option<usize>,
    label_index: Option<usize>,
    epsilon: f64,
    model_snapshot: Vec<Box<[Numeric]>>,
}

impl ModelTrainer for EditedKNearestNeighborTrainer {
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
                println!("Set epsilon to {}", self.epsilon);
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
        let mut training_data = self.model_snapshot.clone();
        training_data.extend(self.training_data.as_ref().unwrap().iter().cloned());
        println!("{:?}", training_data.len());

        let mut model = KNearestNeighbor {
            num_neighbors: self.num_neighbors.ok_or("no num_neighbors")?,
            label_index: self.label_index.ok_or("no label_index")?,
            label_examples: training_data.clone(),
        };

        // Predict values and if the label doesn't match add the input value to the set
        for (idx, sample) in training_data.iter().enumerate().rev() {
            // Remove current sample from list of label examples
            model.label_examples.remove(idx);

            // Predict value of current sample with te rest of the data set
            let prediction = model.predict(sample.clone());

            if (prediction - sample[model.label_index]).abs() > self.epsilon {
                // Sample was predicted incorrectly, therefore the sample is essential to the set
                // and we must add it back to the set
                model.label_examples.push(sample.clone());
            }
        }

        self.model_snapshot = model.label_examples.clone();

        println!("{:?}", self.model_snapshot.len());

        model.generate_voronoi_diagram()?;

        Ok(Box::new(model))
    }
}
