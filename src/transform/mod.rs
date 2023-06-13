//! This module contains logic that transform items in a Numeric column

mod discretization;
mod zscore;
mod log10;

use crate::config::TransformStageConfigs;

use crate::data::data_frame::DataFrame;
use crate::data::column::Column;
use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

pub trait Transform {
    fn apply(
        column: &mut Column<Numeric>,
        parameters: &Option<HashMap<String, Numeric>>,
    ) -> Result<(), Box<dyn Error>>;
}

type TransformFnPtr = fn(&mut Column<Numeric>, &Option<HashMap<String, Numeric>>) -> Result<(), Box<dyn Error>>;

pub fn get_transform(name: &str) -> Result<TransformFnPtr, Box<dyn Error>> {
    match name {
        "equal-frequency-discretization" => Ok(discretization::EqualFrequencyDiscretization::apply),
        "equal-width-discretization" => Ok(discretization::EqualWidthDiscretization::apply),
        "zscore" => Ok(zscore::ZScoreNormalization::apply),
        "log10" => Ok(log10::Log10::apply),
        _ => Err("Invalid transform name given: {name}".into()),
    }
}

pub fn apply(
    table: &mut DataFrame<Numeric>,
    parameters: Vec<TransformStageConfigs>
) -> Result<(), Box<dyn Error>> {
    for parameter in parameters.into_iter() {
        let transform = get_transform(parameter.name.as_str())?;
        if let Some(column) = table.get_column_idx_mut(parameter.index) {
            transform(column, &parameter.parameters)?;
        }
    }

    Ok(())
}
