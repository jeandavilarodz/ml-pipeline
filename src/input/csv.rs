// csv.rs

/// This file contains the implementation for reading CSV formatted tabular data

use super::Reader;
use crate::data::column::Column;
use crate::data::data_frame::DataFrame;
use csv;
use std::error::Error;

pub struct CsvReader;

impl Reader for CsvReader {
    fn read(
        address: &str,
        missing_values: &Vec<String>,
        headers: bool,
    ) -> Result<DataFrame<Option<String>>, Box<dyn Error>> {
        // Read input file, it errors when the file doesn't exist
        let mut reader = csv::Reader::from_path(address)?;

        // Check the headers (first row) for the number of columns and create a column for each
        let first = reader.headers()?;
        let mut columns: Vec<Column<Option<String>>> = Vec::with_capacity(first.len());
        for _ in 0..first.len() {
            columns.push(Column::new());
        }

        // If there are headers present (given by user) then set the header of each column to be
        // the value present in the first row
        if headers {
            first
                .iter()
                .zip(columns.iter_mut())
                .for_each(|(header, col)| col.set_name(header.trim().to_owned()));
        }

        // For each record in the input file, push the value to the corresponding column
        for rec in reader.records() {
            // For each value in the row, push to an appropriate column
            for (entry, col) in rec?.iter().zip(columns.iter_mut()) {
                // Trim the whitespace of the entry
                let entry = entry.trim().to_owned();
                if missing_values.contains(&entry) {
                    // The entry is in the list of missing values, therefore it should be marked as none
                    col.push(None);
                } else {
                    // Push the entry as some value
                    col.push(Some(entry));
                }
            }
        }

        // Build the table by adding each parsed column
        let mut ret = DataFrame::new();
        columns.into_iter().for_each(|col| ret.add_column(col));
        Ok(ret)
    }
}
