//! This module contains logic that transform items in a Numeric column

mod discretization;
mod normalize;

use crate::data::column::Column;
use crate::data::data_frame::DataFrame;
use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

use lazy_static::lazy_static;

pub trait Transform {
    fn apply(&self, column: &mut Column<Numeric>, parameters: &Option<HashMap<&str, Numeric>>);
}

lazy_static! {
    static ref TRANSFORM_TYPE_MAP: HashMap<&'static str, Box<dyn Transform + Sync>> =
        HashMap::from([
            (
                "normalize",
                Box::new(normalize::NormalizeTransform) as Box<dyn Transform + Sync>
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
    transforms: Vec<(&str, usize)>,
    parameters: Option<HashMap<&str, Numeric>>,
) -> Result<(), Box<dyn Error>> {
    let missing_transform = transforms
        .iter()
        .fold(false, |acc, p| acc | !TRANSFORM_TYPE_MAP.contains_key(p.0));

    if missing_transform {
        simple_error::bail!("Parser type not supported!");
    }

    for (transform, idx) in transforms.into_iter() {
        let operation = &TRANSFORM_TYPE_MAP[transform];
        if let Some(column) = table.get_column_idx_mut(idx) {
            operation.apply(column,&parameters);
        }
    }

    Ok(())
}
