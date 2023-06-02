use super::Transform;
use crate::data::column::Column;
use crate::types::Numeric;

use std::collections::HashMap;

pub struct EqualWidthDiscretization;

impl Transform for EqualWidthDiscretization {
    fn apply(&self, column: &mut Column<Numeric>, parameters: &Option<HashMap<&str, Numeric>>) {
        if parameters.is_none() {
            return;
        }

        let parameters = parameters.as_ref().unwrap();

        if !parameters.contains_key("num_bins") {
            return;
        }

        let biggest = *column
            .values_mut()
            .max_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap())
            .unwrap();
        let smallest = *column
            .values()
            .min_by(|x, y| y.abs().partial_cmp(&x.abs()).unwrap())
            .unwrap();

        let bin_range = (biggest - smallest) / parameters["num_bins"];

        for value in column.values_mut() {
            *value = bin_range * ((*value - smallest) / bin_range).floor()
                + smallest
                + (bin_range / 2.0);
        }
    }
}

pub struct EqualFrequencyDiscretization;

impl Transform for EqualFrequencyDiscretization {
    fn apply(&self, column: &mut Column<Numeric>, parameters: &Option<HashMap<&str, Numeric>>) {
        if parameters.is_none() {
            return;
        }

        let parameters = parameters.as_ref().unwrap();

        if !parameters.contains_key("num_bins") {
            return;
        }

        let len_items = column.values().len();
        let num_bins = parameters["num_bins"] as usize;
        let max_items_per_bin = len_items / num_bins;

        if max_items_per_bin < 1 {
            return;
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
                .fold(0.0, |acc,(_ , val)| acc + val)
                / max_items_per_bin as Numeric;
            for (idx, _) in sorted[bin_index_start..bin_index_end].iter_mut() {
                *(column.get_mut(*idx).unwrap()) = mean;
            }
        }
        if len_items % 2 == 1 {
            let last_mean = sorted[(len_items - (max_items_per_bin + 1))..len_items]
                .iter()
                .fold(0.0, |acc, val| acc + val.1)
                / (max_items_per_bin + 1) as Numeric;
            for idx in (len_items - (max_items_per_bin + 1))..len_items {
                *(column.get_mut(idx).unwrap()) = last_mean;
            }
        }
    }
}
