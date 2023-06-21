// knn_condensed.rs

//! This file implements the logic to train a condensed k-nearest neighbor learner

use super::Model;
use super::ModelTrainer;

use crate::models::knn_classifier::KNearestNeighbor;
use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

use itertools::Itertools;
use plotly::color::Rgb;
use plotly::common::{Fill, Marker, Mode, Title};
use plotly::layout::Axis;
use plotly::{Layout, Plot, Scatter};

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
        generate_voronoi_diagram(&self.model_snapshot)?;

        Ok(Box::new(model))
    }
}

// This function takes a vector of training data and generate the voronoi diagram
// using plotters
fn generate_voronoi_diagram(
    points: &Vec<Box<[Numeric]>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let index = (0, 3);
    let voronoi_points = points
        .iter()
        .cloned()
        .map(|p| voronoi::Point::new(p[index.0], p[index.1]))
        .collect_vec();
    let diagram = voronoi::voronoi(voronoi_points, 10.0);
    let polygons = voronoi::make_polygons(&diagram);

    let mut triangles = Vec::new();
    for polygon in polygons {
        println!("Polygon: {:?}", polygon);
        let (x, y) = polygon.into_iter().map(|p| (p.x(), p.y())).unzip();
        triangles.push(
            Scatter::new(x, y)
                .fill(Fill::None)
                .mode(Mode::Lines)
                .marker(Marker::new().color(Rgb::new(0, 0, 0)))
        );
    }

    let mut plot = Plot::new();
    for triangle in triangles {
        plot.add_trace(triangle);
    }

    let (sx, sy) = points
        .iter()
        .cloned()
        .map(|p| (p[index.0], p[index.1]))
        .unzip();
    let class_labels = points
        .iter()
        .cloned()
        .map(|p| format!("class: {}", p.last().unwrap().to_string()))
        .collect_vec();

    plot.add_trace(
        Scatter::new(sx, sy)
            .mode(Mode::Markers)
            .marker(Marker::new().color(Rgb::new(0, 0, 0)))
            .name("Reference Points")
            .text_array(class_labels),
    );
    let layout = Layout::new()
        .title(Title::new("Data Labels Hover"))
        .x_axis(Axis::new().title(Title::new(&format!("x[{}]", index.0))))
        .y_axis(Axis::new().title(Title::new(&format!("x[{}]", index.1))));
    plot.set_layout(layout);

    plot.show();

    Ok(())
}
