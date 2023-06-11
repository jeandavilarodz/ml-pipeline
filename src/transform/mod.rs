//! This module contains logic that transform items in a Numeric column

mod discretization;
mod zscore;

use crate::config::TransformStageConfigs;
use crate::data::data_frame::DataFrame;
use crate::data::column::Column;
use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

use lazy_static::lazy_static;

pub trait Transform {
    fn apply(
        &self,
        column: &mut Column<Numeric>,
        parameters: &Option<HashMap<String, Numeric>>,
    ) -> Result<(), Box<dyn Error>>;
}

lazy_static! {
    static ref TRANSFORM_TYPE_MAP: HashMap<&'static str, Box<dyn Transform + Sync>> =
        HashMap::from([
            (
                "zscore",
                Box::new(zscore::ZScoreNormalization) as Box<dyn Transform + Sync>
            ),
            (
                "equal-width-discretization",
                Box::new(discretization::EqualWidthDiscretization) as Box<dyn Transform + Sync>
            ),
            (
                "equal-frequency-discretization",
                Box::new(discretization::EqualFrequencyDiscretization) as Box<dyn Transform + Sync>
            )
        ]);
}

pub fn apply(
    table: &mut DataFrame<Numeric>,
    parameters: Vec<TransformStageConfigs>
) -> Result<(), Box<dyn Error>> {

    println!("{:?}", parameters);

    let missing_transform = parameters
        .iter()
        .fold(false, |acc, p| acc | !TRANSFORM_TYPE_MAP.contains_key(p.name.as_str()));

    if missing_transform {
        return Err("Transform type not supported!".into());
    }

    for parameter in parameters.into_iter() {
        let operation = &TRANSFORM_TYPE_MAP[parameter.name.as_str()];
        if let Some(column) = table.get_column_idx_mut(parameter.index) {
            operation.apply(column, &parameter.parameters)?;
        }
    }

    Ok(())
}
