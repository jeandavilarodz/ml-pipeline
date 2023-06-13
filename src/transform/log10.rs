// log10.rs

/// This file describes a transform to do a log10 of the values in the column

use super::Transform;
use crate::data::column::Column;
use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

pub struct Log10;

impl Transform for Log10 {
    fn apply(
        column: &mut Column<Numeric>,
        parameters: &Option<HashMap<String, Numeric>>,
    ) -> Result<(), Box<dyn Error>> {
        let offset = if let Some(parameters) = parameters {
            if let Some(input) = parameters.get("offset") {
                *input
            }
            else {
                0.0
            }
        } else {0.0};

        for value in column.values_mut() {
            (*value) = (*value + offset).log10();
        }

        Ok(())
    }
}
