//! This contains logic to read input

pub mod csv;

use lazy_static::lazy_static;

use crate::config::InputStageConfigs;
use crate::data::data_frame::DataFrame;
use std::collections::HashMap;
use std::error::Error;

pub trait Reader {
    fn read(
        &self,
        address: &str,
        missing_values: &Vec<String>,
        headers: bool,
    ) -> Result<DataFrame<Option<String>>, Box<dyn Error>>;
}

lazy_static! {
    static ref INPUT_FORMAT_MAP: HashMap<&'static str, Box<dyn Reader + Sync>> =
        HashMap::from([("csv", Box::new(csv::CsvReader) as Box<dyn Reader + Sync>),]);
}

pub fn read_input(
    parameters: InputStageConfigs,
) -> Result<DataFrame<Option<String>>, Box<dyn Error>> {
    let format = parameters.format.as_str();
    if !INPUT_FORMAT_MAP.contains_key(format) {
        return Err("Invalid input format!".into());
    }
    INPUT_FORMAT_MAP[format].read(
        &parameters.address,
        &parameters.missing_values,
        parameters.headers,
    )
}
