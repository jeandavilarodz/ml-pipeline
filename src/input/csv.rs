use super::Reader;
use crate::data::data_frame::DataFrame;
use crate::data::column::Column;
use std::error::Error;
use csv;

pub struct CsvReader;

impl Reader for CsvReader {
    fn read(&self, address: &str, missing_values: Vec<&str>, headers: bool) -> Result<DataFrame<Option<&str>>, Box<dyn Error>> {
        let mut reader = csv::Reader::from_path(address)?;
        let headers = reader.headers()?;
        let mut columns: Vec<Column<&str>> = Vec::with_capacity(headers.len());
        for _ in 0..headers.len() {
            columns.push(Column::new());
        }
        if headers {
            headers.iter().zip(columns.iter_mut()).for_each(|(header, col)| col.set_name(header));
        }
        for rec in reader.records() {
            for (entry, col) in rec?.iter().zip(columns.iter_mut()) {
                if missing_values.contains(entry) {
                    None
                } 
                else {
                    col.push(entry)
                }
            }
        }
        let mut ret = DataFrame::new();
        columns.into_iter().for_each(|col| ret.add_column(col));
        Ok(ret)
    }
}