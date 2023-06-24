// knn_classifier.rs

//! This file implements the logic to predict values using a k-nearest neighbor learner

use super::Model;

use crate::types::{Numeric, NUMERIC_DIGIT_PRECISION};

use std::collections::HashMap;

use plotly::color::Rgb;
use plotly::common::{Fill, Marker, Mode, Orientation, Position, Title};
use plotly::layout::{Axis, Legend};
use plotly::{Layout, Plot, Scatter};

pub struct KNearestNeighbor {
    pub label_examples: Vec<Box<[Numeric]>>,
    pub label_index: usize,
    pub num_neighbors: usize,
}

impl Model for KNearestNeighbor {
    fn predict(&self, sample: &[Numeric]) -> Numeric {
        // Calculate distances between each example and the k nearest neighbors
        let mut distances = Vec::new();
        for example in self.label_examples.iter() {
            distances.push((example, euclidean_distance(example, sample)));
        }
        // Sort the distances by distance
        distances.sort_by(|(_, x), (_, y)| x.abs().partial_cmp(&y.abs()).unwrap());

        // Get the label count of the k nearest neighbors
        let mut label_vote = HashMap::new();
        distances
            .iter()
            .take(self.num_neighbors)
            .for_each(|(neighbor, _)| {
                let key = (neighbor[self.label_index] / NUMERIC_DIGIT_PRECISION) as i64;
                let counter = label_vote.entry(key).or_insert(0);
                *counter += 1;
            });

        // Get the most common label
        let mode = label_vote
            .iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .expect("No mode found!");

        // return the most common label
        (*mode as f64) * NUMERIC_DIGIT_PRECISION
    }

    fn type_id(&self) -> &'static str {
        "KNearestNeighbor"
    }

    fn get_hyperparameters(&self) -> HashMap<String, String> {
        let mut ret = HashMap::from([
            ("label_index".into(), self.label_index.to_string()),
            ("num_neighbors".into(), self.num_neighbors.to_string()),    
        ]);
        self.label_examples.iter().enumerate().for_each(|(idx, ex)| {
            ret.insert(format!("label_example_{}", idx), ex.iter().fold("".to_string(), |acc, v| acc + &format!(",{}", v)));
        });
        return ret;
    }
}

fn euclidean_distance(row1: &[Numeric], row2: &[Numeric]) -> Numeric {
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
        let points = self.label_examples.as_slice();
        let index = (0, 3);
        let voronoi_points = points
            .iter()
            .map(|p| voronoi::Point::new(p[index.0], p[index.1]))
            .collect();
        let diagram = voronoi::voronoi(voronoi_points, 10.0);
        let polygons = voronoi::make_polygons(&diagram);

        let mut voronoi_cells = Vec::new();
        for polygon in polygons.iter() {
            let (x, y) = polygon.iter().map(|p| (p.x(), p.y())).unzip();
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

        let (sx, sy) = points.iter().map(|p| (p[index.0], p[index.1])).unzip();
        let class_labels = points
            .iter()
            .map(|p| *p.last().unwrap())
            .collect::<Vec<f64>>();

        plot.add_trace(
            Scatter::new(sx, sy)
                .mode(Mode::MarkersText)
                .text_position(Position::TopCenter)
                .marker(Marker::new().color(Rgb::new(0, 0, 0)))
                .name("Reference Points")
                .text_array(class_labels.iter().map(|s| s.to_string()).collect()),
        );
        let layout = Layout::new()
            .title(Title::new("Voronoi Diagram Between Reference Points"))
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
