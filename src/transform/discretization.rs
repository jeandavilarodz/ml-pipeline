// discretization.rs

/// This file contains the logic to implement two strategies for discretization of the input features. 
/// The first strategy is equal-width discretization, where the data is split into a fixed number of
/// bins. The second strategy is equal-frequency discretization, where the data is split into a
/// fixed number of bins, but the number of items per bin is kept fixed.

use num_traits::ToPrimitive;

use super::Transform;
use crate::data::column::Column;
use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

pub struct EqualWidthDiscretization;

impl Transform for EqualWidthDiscretization {
    fn apply(
        column: &mut Column<Numeric>,
        parameters: &Option<HashMap<String, Numeric>>,
    ) -> Result<(), Box<dyn Error>> {
        let parameters = parameters.as_ref().ok_or("No parameters given!")?;
        let num_bins = parameters
            .get("num_bins")
            .ok_or("num_bins parameter not present!")?
            .to_usize()
            .ok_or("Could not parse num_bins as usize!")?;

        if num_bins < 2 {
            return Err("Number of bins is less than 2!".into());
        }

        let biggest = *column
            .values_mut()
            .max_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap())
            .unwrap();
        let smallest = *column
            .values()
            .min_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap())
            .unwrap();

        let num_bins = num_bins - 1;

        let bin_range = (biggest - smallest) / num_bins as f64;

        for value in column.values_mut() {
            *value = bin_range * (((*value - smallest) / bin_range).floor()) + smallest;
        }

        Ok(())
    }
}

pub struct EqualFrequencyDiscretization;

impl Transform for EqualFrequencyDiscretization {
    fn apply(
        column: &mut Column<Numeric>,
        parameters: &Option<HashMap<String, Numeric>>,
    ) -> Result<(), Box<dyn Error>> {
        let parameters = parameters.as_ref().ok_or("No parameters given!")?;
        let num_bins = parameters
            .get("num_bins")
            .ok_or("num_bins parameter not present!")?
            .to_usize()
            .ok_or("Could not parse num_bins as usize!")?;

        if num_bins < 2 {
            return Err("Number of bins is less than 2!".into());
        }

        let len_items = column.values().len();
        let max_items_per_bin = len_items / num_bins;

        if max_items_per_bin < 1 {
            return Err("Number of items per bin is less than 1!".into());
        }

        let mut sorted = column
            .values()
            .copied()
            .enumerate()
            .collect::<Vec<(usize, Numeric)>>();
        sorted.sort_by(|(_, x), (_, y)| x.abs().partial_cmp(&y.abs()).unwrap());

        for bin_number in 0..num_bins {
            let bin_index_start = bin_number * max_items_per_bin;
            let bin_index_end = bin_index_start + max_items_per_bin;
            let mean = sorted[bin_index_start..bin_index_end]
                .iter()
                .fold(0.0, |acc, (_, val)| acc + val)
                / max_items_per_bin as f64;
            for (idx, _) in sorted[bin_index_start..bin_index_end].iter_mut() {
                *(column.get_mut(*idx).unwrap()) = mean;
            }
        }
        if len_items % 2 == 1 {
            let last_mean = sorted[(len_items - (max_items_per_bin + 1))..len_items]
                .iter()
                .fold(0.0, |acc, val| acc + val.1)
                / (max_items_per_bin + 1) as f64;
            for (idx, _) in sorted[(len_items - (max_items_per_bin + 1))..len_items].iter() {
                *(column.get_mut(*idx).unwrap()) = last_mean;
            }
        }
        Ok(())
    }
}
