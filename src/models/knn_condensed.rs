// knn_condensed.rs

/// This file implements the logic to train and predict values using a condensed
/// k-nearest neighbor learner
use super::Model;

use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

use num_traits::ToPrimitive;

pub struct CondensedKNN {
    training_set: Vec<Box<[Numeric]>>,
    training_index: usize,
    num_neighbors: usize,
}

impl Model for CondensedKNN {
    fn from_parameters(
        parameters: &Option<HashMap<String, Numeric>>,
    ) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        let parameters = parameters.as_ref().ok_or("No parameters given!")?;
        let num_neighbors = parameters
            .get("num_neighbors")
            .ok_or("num_neighbors parameter not present!")?
            .to_usize()
            .ok_or("Could not parse num_bins as usize!")?;
        Ok(Self {
            training_index: 0,
            training_set: vec![],
            num_neighbors: num_neighbors,
        })
    }

    fn predict(&self, sample: Box<[Numeric]>) -> Result<Numeric, Box<dyn Error>> {
        let mut distances = Vec::new();
        for (index, training_sample) in self.training_set.iter().enumerate() {
            distances.push((
                index,
                euclidean_distance(training_sample.clone(), sample.clone())?,
            ));
        }
        distances.sort_by(|(_, x), (_, y)| x.abs().partial_cmp(&y.abs()).unwrap());

        let mut label_count = HashMap::new();
        for (idx, _) in distances[..self.num_neighbors].into_iter() {
            let label = self.training_set[*idx]
                .get(self.training_index)
                .ok_or("Couldn't get label from training data")?;
            let key = (label * 1e8) as i64;
            let counter = label_count.entry(key).or_insert(0);
            *counter += 1;
        }

        let mode = label_count
            .iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .ok_or("No maximum found in the histogram!")?;

        Ok((*mode as f64) * 1e-8)
    }

    fn train(
        &mut self,
        training_values: &Vec<Box<[Numeric]>>,
        target_value_idx: usize,
    ) -> Result<(), Box<dyn Error>> {
        if training_values.len() < 1 {
            return Err("Empty training set given!".into());
        }
        if training_values.get(target_value_idx).is_none() {
            return Err("Could not find target label in training data!".into());
        }

        // Predict values and if the label doesn't match add the input value to the set
        for sample in training_values.iter() {
            let prediction = self.predict(sample.clone())?;
            if (prediction - sample[target_value_idx]).abs() < 1e-8 {
                self.training_set.push(sample.clone());
            }
        }

        self.training_index = target_value_idx;

        Ok(())
    }
}

fn euclidean_distance(row1: Box<[Numeric]>, row2: Box<[Numeric]>) -> Result<f64, Box<dyn Error>> {
    if row1.len() != row2.len() {
        return Err("Samples do not contain the same number of features!".into());
    }
    let distance = row1
        .iter()
        .zip(row2.iter())
        .fold(0.0, |acc, (&e1, &e2)| acc + (e1 - e2) * (e1 - e2));
    Ok(distance.sqrt())
}
