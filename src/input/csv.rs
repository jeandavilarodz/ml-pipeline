use super::Reader;
use crate::data::column::Column;
use crate::data::data_frame::DataFrame;
use csv;
use std::error::Error;

pub struct CsvReader;

impl Reader for CsvReader {
    fn read(
        &self,
        address: &str,
        missing_values: &[&str],
        headers: bool,
    ) -> Result<DataFrame<Option<String>>, Box<dyn Error>> {
        let mut reader = csv::Reader::from_path(address)?;
        let first = reader.headers()?;
        let mut columns: Vec<Column<Option<String>>> = Vec::with_capacity(first.len());
        for _ in 0..first.len() {
            columns.push(Column::new());
        }
        if headers {
            first
                .iter()
                .zip(columns.iter_mut())
                .for_each(|(header, col)| col.set_name(header.trim().to_owned()));
        }
        for rec in reader.records() {
            for (entry, col) in rec?.iter().zip(columns.iter_mut()) {
                if missing_values.contains(&entry) {
                    col.push(None);
                } else {
                    col.push(Some(entry.trim().to_owned()));
                }
            }
        }
        let mut ret = DataFrame::new();
        columns.into_iter().for_each(|col| ret.add_column(col));
        Ok(ret)
    }
}
