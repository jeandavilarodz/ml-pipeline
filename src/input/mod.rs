//! This contains logic to read input

pub mod csv;

use simple_error::bail;
use lazy_static::lazy_static;

use crate::data::data_frame::DataFrame;
use std::collections::HashMap;
use std::error::Error;

pub trait Reader {
    fn read(&self, address: &str) -> Result<DataFrame<String>, Box<dyn Error>>;
    fn with_headers(&self, address: &str) -> Result<DataFrame<String>, Box<dyn Error>>;
}

lazy_static!{
static ref INPUT_FORMAT_MAP: HashMap<&'static str, Box<dyn Reader + Sync>> = HashMap::from([
    ("csv", Box::new(csv::CsvReader) as Box<dyn Reader + Sync>),
]);
}

pub fn read(address: &str, format: &str) -> Result<DataFrame<String>, Box<dyn Error>> {
    if !INPUT_FORMAT_MAP.contains_key(format) {
        bail!("Invalid input format!");
    }
    INPUT_FORMAT_MAP[format].read(address)
}

pub fn with_headers(address: &str, format: &str) -> Result<DataFrame<String>, Box<dyn Error>> {
    if !INPUT_FORMAT_MAP.contains_key(format) {
        bail!("Invalid input format!");
    }
    INPUT_FORMAT_MAP[format].with_headers(address)
}