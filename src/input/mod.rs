//! This contains logic to read input into a table data structure

mod csv;

use crate::data::data_frame::DataFrame;
use std::error::Error;

pub trait Reader {
    fn read(
        address: &str,
        missing_values: &[String],
        headers: bool,
    ) -> Result<DataFrame<Option<String>>, Box<dyn Error>>;
}

type InputFnPtr = fn(&str, &[String], bool) -> Result<DataFrame<Option<String>>, Box<dyn Error>>;

pub fn get_reader(format: &str) -> Result<InputFnPtr, Box<dyn Error>> {
    match format {
        "csv" => Ok(csv::CsvReader::read),
        _ => Err("Invalid format passed!".into()),
    }
}
