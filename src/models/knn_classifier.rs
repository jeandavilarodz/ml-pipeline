// knn_classifier.rs

//! This file implements the logic to predict values using a k-nearest neighbor learner

use super::Model;

use crate::types::Numeric;

use std::collections::HashMap;

pub struct KNearestNeighbor {
    pub label_examples: Vec<Box<[Numeric]>>,
    pub label_index: usize,
    pub num_neighbors: usize,
}

impl Model for KNearestNeighbor {
    fn predict(&self, sample: Box<[Numeric]>) -> Numeric {
        let mut distances = Vec::new();
        for (index, training_sample) in self.label_examples.iter().enumerate() {
            distances.push((
                index,
                euclidean_distance(training_sample.clone(), sample.clone()),
            ));
        }
        distances.sort_by(|(_, x), (_, y)| x.abs().partial_cmp(&y.abs()).unwrap());

        let mut label_count = HashMap::new();
        for (idx, _) in distances[..self.num_neighbors].into_iter() {
            let label = self.label_examples[*idx][self.label_index];
            let key = (label * 1e8) as i64;
            let counter = label_count.entry(key).or_insert(0);
            *counter += 1;
        }

        let mode = label_count
            .iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .expect("No mode found!");

        (*mode as f64) * 1e-8
    }
}

fn euclidean_distance(row1: Box<[Numeric]>, row2: Box<[Numeric]>) -> Numeric {
    let distance = row1
        .iter()
        .zip(row2.iter())
        .fold(0.0, |acc, (&e1, &e2)| acc + (e1 - e2).powi(2));
    distance.sqrt()
}