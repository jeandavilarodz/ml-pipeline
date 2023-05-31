//! This contains logic to read input

pub mod csv;

use simple_error::bail;
use lazy_static::lazy_static;

use crate::data::data_frame::DataFrame;
use std::collections::HashMap;
use std::error::Error;

pub trait Reader {
    fn read(&self, address: &str, missing_values: &Vec<&str>, headers: bool) -> Result<DataFrame<Option<&str>>, Box<dyn Error>>;
}

lazy_static!{
    static ref INPUT_FORMAT_MAP: HashMap<&'static str, Box<dyn Reader + Sync>> = HashMap::from([
        ("csv", Box::new(csv::CsvReader) as Box<dyn Reader + Sync>),
    ]);
}

pub fn read_input(address: &str, format: &str, missing_values: Vec<&str>, headers: bool) {
    if !INPUT_FORMAT_MAP.contains_key(format) {
        bail!("Invalid input format!");
    }
    INPUT_FORMAT_MAP[format].read(address, &missing_values, headers)
}