// knn_classifier.rs

//! This file implements the logic to predict values using a k-nearest neighbor learner

use super::Model;

use crate::types::Numeric;

use std::collections::HashMap;

use itertools::Itertools;
use plotly::color::Rgb;
use plotly::common::{Fill, Marker, Mode, Title, Position, Orientation};
use plotly::layout::{Axis, Legend};
use plotly::{Layout, Plot, Scatter};

pub struct KNearestNeighbor {
    pub label_examples: Vec<Box<[Numeric]>>,
    pub label_index: usize,
    pub num_neighbors: usize,
}

impl Model for KNearestNeighbor {
    fn predict(&self, sample: Box<[Numeric]>) -> Numeric {
        // Calculate distances between each example and the k nearest neighbors
        let mut distances = Vec::new();
        for (index, training_sample) in self.label_examples.iter().enumerate() {
            distances.push((
                index,
                euclidean_distance(training_sample.clone(), sample.clone()),
            ));
        }
        // Sort the distances by distance
        distances.sort_by(|(_, x), (_, y)| x.abs().partial_cmp(&y.abs()).unwrap());

        // Get the label count of the k nearest neighbors
        let mut label_count = HashMap::new();
        for (idx, _) in distances[..self.num_neighbors].into_iter() {
            let label = self.label_examples[*idx][self.label_index];
            let key = (label * 1e8) as i64;
            let counter = label_count.entry(key).or_insert(0);
            *counter += 1;
        }

        // Get the most common label
        let mode = label_count
            .iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .expect("No mode found!");

        // return the most common label
        (*mode as f64) * 1e-8
    }
}

fn euclidean_distance(row1: Box<[Numeric]>, row2: Box<[Numeric]>) -> Numeric {
    // Calculate the euclidean distance between the two rows
    let distance = row1
        .iter()
        .zip(row2.iter())
        .fold(0.0, |acc, (&e1, &e2)| acc + (e1 - e2).powi(2));

    // Return the distance
    distance.sqrt()
}

impl KNearestNeighbor {
    // This function takes a vector of training data and generate the voronoi diagram
    // using plotters
    pub fn generate_voronoi_diagram(&self) -> Result<(), Box<dyn std::error::Error>> {
        let points = &self.label_examples;
        let index = (0, 3);
        let voronoi_points = points
            .iter()
            .cloned()
            .map(|p| voronoi::Point::new(p[index.0], p[index.1]))
            .collect_vec();
        let diagram = voronoi::voronoi(voronoi_points, 10.0);
        let polygons = voronoi::make_polygons(&diagram);

        let mut voronoi_cells = Vec::new();
        for polygon in polygons {
            let (x, y) = polygon.into_iter().map(|p| (p.x(), p.y())).unzip();
            voronoi_cells.push(
                Scatter::new(x, y)
                    .fill(Fill::None)
                    .mode(Mode::Lines)
                    .marker(Marker::new().color(Rgb::new(0, 0, 0)))
                    .show_legend(false),
            );
        }

        let mut plot = Plot::new();
        for cell in voronoi_cells.into_iter() {
            plot.add_trace(cell);
        }

        let (sx, sy) = points
            .iter()
            .cloned()
            .map(|p| (p[index.0], p[index.1]))
            .unzip();
        let class_labels = points
            .iter()
            .cloned()
            .map(|p| *p.last().unwrap())
            .collect_vec();

        plot.add_trace(
            Scatter::new(sx, sy)
                .mode(Mode::MarkersText)
                .text_position(Position::TopCenter)
                .marker(Marker::new().color(Rgb::new(0, 0, 0)))
                .name("Reference Points")
                .text_array(
                    class_labels
                        .iter()
                        .cloned()
                        .map(|s| s.to_string())
                        .collect_vec(),
                ),
        );
        let layout = Layout::new()
            .title(Title::new("Data Labels Hover"))
            .x_axis(
                Axis::new()
                    .title(Title::new(&format!("x[{}]", index.0)))
                    .show_grid(true),
            )
            .y_axis(
                Axis::new()
                    .title(Title::new(&format!("x[{}]", index.1)))
                    .show_grid(true),
            )
            .legend(Legend::new().orientation(Orientation::Horizontal))
            .height(800)
            .width(800);
        plot.set_layout(layout);

        plot.show();

        Ok(())
    }
}
