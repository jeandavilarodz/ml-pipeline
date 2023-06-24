// csv.rs

/// This file contains the implementation for reading CSV formatted tabular data

use super::Reader;
use crate::data::column::Column;
use crate::data::data_frame::DataFrame;

use std::error::Error;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs::File;

pub struct CsvReader;

impl Reader for CsvReader {
    fn read(
        address: &str,
        missing_values: &[String],
        headers: bool,
    ) -> Result<DataFrame<Option<String>>, Box<dyn Error>> {
        let path = Path::new(address);
        if !path.exists() {
            return Err(format!("Specified file does not exist! ({})", address).into());
        }
        let file = File::open(path)?;
        let mut reader = BufReader::new(file).lines();

        // Check the headers (first row) for the number of columns and create a column for each
        let first_line = reader.next().ok_or("Couldn't read first line!")??;
        let first_fields: Vec<_> = first_line.split(',').collect();
        let mut columns: Vec<Column<Option<String>>> = Vec::with_capacity(first_fields.len());
        for _ in 0..first_fields.len() {
            columns.push(Column::new());
        }

        // If there are headers present (given by user) then set the header of each column to be
        // the value present in the first row
        if headers {
            first_fields
                .iter()
                .zip(columns.iter_mut())
                .for_each(|(header, col)| col.set_name(header.trim().to_owned()));
        }
        else {
            // Need to do this because the headers() call will remove the first row from the iterator
            for (entry, col) in first_fields.iter().zip(columns.iter_mut()) {
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

        // For each record in the input file, push the value to the corresponding column
        for line in reader {
            let line = line?;
            // For each value in the row, push to an appropriate column
            for (entry, col) in line.split(',').zip(columns.iter_mut()) {
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
