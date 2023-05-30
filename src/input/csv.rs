use super::Reader;
use crate::data::data_frame::DataFrame;
use crate::data::column::Column;
use std::error::Error;
use csv;

pub struct CsvReader;

impl Reader for CsvReader {
    fn with_headers(&self, address: &str) -> Result<DataFrame<String>, Box<dyn Error>> {
        let mut reader = csv::Reader::from_path(address)?;
        let headers = reader.headers()?;
        let mut columns: Vec<Column<String>> = Vec::with_capacity(headers.len());
        for _ in 0..headers.len() {
            columns.push(Column::new());
        }
        headers.iter().zip(columns.iter_mut()).for_each(|(header, col)| {
            col.set_name(header.to_owned())
        });
        for rec in reader.records() {
            rec?.iter().zip(columns.iter_mut()).for_each(|(entry, col)| col.push(entry.to_owned()));
        }
        let mut ret = DataFrame::new();
        columns.into_iter().for_each(|col| ret.add_column(col));
        Ok(ret)
    }
    
    fn read(&self, address: &str) -> Result<DataFrame<String>, Box<dyn Error>> {
        let mut reader = csv::Reader::from_path(address)?;
        let headers = reader.headers()?;
        let mut columns: Vec<Column<String>> = Vec::with_capacity(headers.len());
        for _ in 0..headers.len() {
            columns.push(Column::new());
        }
        for rec in reader.records() {
            rec?.iter().zip(columns.iter_mut()).for_each(|(entry, col)| col.push(entry.to_owned()));
        }
        let mut ret = DataFrame::new();
        columns.into_iter().for_each(|col| ret.add_column(col));
        Ok(ret)
    }
}