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
        // Check if parameters were given and are correct
        let parameters = parameters.as_ref().ok_or("No parameters given!")?;
        let num_bins = parameters
            .get("num_bins")
            .ok_or("num_bins parameter not present!")?
            .to_usize()
            .ok_or("Could not parse num_bins as usize!")?;

        if num_bins < 2 {
            return Err("Number of bins is less than 2!".into());
        }

        // Calculate the smallest and largest value to divide the range into equal width values
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

        // This is a complicated function that maps all values into the smallest value
        // that corresponds to the range. Prof said this equation was okay in email.
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
        // Check that parameters exist and are correct
        let parameters = parameters.as_ref().ok_or("No parameters given!")?;
        let num_bins = parameters
            .get("num_bins")
            .ok_or("num_bins parameter not present!")?
            .to_usize()
            .ok_or("Could not parse num_bins as usize!")?;

        if num_bins < 2 {
            return Err("Number of bins is less than 2!".into());
        }

        // Calculate largest number of values in a bin
        let len_items = column.values().len();
        let mut max_items_per_bin = len_items / num_bins;

        if max_items_per_bin < 1 {
            return Err("Number of items per bin is less than 1!".into());
        }

        // Copy all values in the column with their respective, enumerated indexes
        let mut sorted = column
            .values()
            .copied()
            .enumerate()
            .collect::<Vec<(usize, Numeric)>>();

        // Sort all items in the copied values
        sorted.sort_by(|(_, x), (_, y)| x.abs().partial_cmp(&y.abs()).unwrap());

        // For all bins split values equally by max_items per bin
        for bin_number in 0..(num_bins - 1) {
            let bin_index_start = bin_number * max_items_per_bin;
            let bin_index_end = bin_index_start + max_items_per_bin;
            let mean = sorted[bin_index_start..bin_index_end]
                .iter()
                .fold(0.0, |acc, (_, val)| acc + val)
                / max_items_per_bin as f64;
            for (idx, _) in sorted[bin_index_start..bin_index_end].iter() {
                let value = column.get_mut(*idx).ok_or("Could not find index of value in column")?;
                *value = mean;
            }
        }

        // Modify the last bin to be extended by the residual values of the sorted array
        // Mostly used for columns of odd length
        max_items_per_bin += max_items_per_bin % len_items;
    
        // Replace all values that correspond to the last bin
        let last_mean = sorted[(len_items - max_items_per_bin)..]
            .iter()
            .fold(0.0, |acc, val| acc + val.1) / (max_items_per_bin as f64);
        for (idx, _) in sorted[(len_items - max_items_per_bin)..len_items].iter() {
            let value = column.get_mut(*idx).ok_or("Could not find index of value in column")?;
            *value = last_mean;
        }
    
        Ok(())
    }
}
