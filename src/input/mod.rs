//! This contains logic to read input

pub mod csv;

use crate::data::data_frame::DataFrame;
use std::error::Error;

pub trait Reader {
    fn read(address: &str) -> Result<DataFrame<String>, Box<dyn Error>>;
    fn with_headers(address: &str) -> Result<DataFrame<String>, Box<dyn Error>>;
}
