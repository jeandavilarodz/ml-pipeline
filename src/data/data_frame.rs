// data_frame.rs

/// This is the data structure that abstracts values in a table. It is composed of multiple
/// column abstractions (in column.rs) and a map that converts column header name into
/// an index into the array of columns. It provides iterators so that algorithms can iterate
/// over the columns and itself provides the abstraction to get a row from the table.

use crate::data::column::Column;

use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub struct DataFrame<T: Sized> {
    columns: Vec<Column<T>>,
    column_idx_map: HashMap<String, usize>,
}

impl<T> Default for DataFrame<T> 
where T: Clone {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DataFrame<T> 
where T: Clone {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            column_idx_map: HashMap::new(),
        }
    }

    pub fn columns(&self) -> std::slice::Iter<'_, Column<T>> {
        self.columns.iter()
    }

    pub fn columns_mut(&mut self) -> std::slice::IterMut<'_, Column<T>> {
        self.columns.iter_mut()
    }

    pub fn add_column(&mut self, column: Column<T>) {
        if let Some(name) = column.get_name() {
            self.column_idx_map
                .insert(name.to_owned(), self.columns.len());
        }
        self.columns.push(column);
    }

    pub fn get_column_idx(&self, idx: usize) -> Option<&Column<T>> {
        self.columns.get(idx)
    }

    pub fn get_column_idx_mut(&mut self, idx: usize) -> Option<&mut Column<T>> {
        self.columns.get_mut(idx)
    }

    pub fn get_column_name(&self, name: &str) -> Option<&Column<T>> {
        match self.column_idx_map.get(name) {
            Some(idx) => self.columns.get(*idx),
            _ => None,
        }
    }

    pub fn get_column_name_mut(&mut self, name: &str) -> Option<&mut Column<T>> {
        match self.column_idx_map.get(name) {
            Some(idx) => self.columns.get_mut(*idx),
            _ => None,
        }
    }

    pub fn get_row(&self, idx: usize) -> Result<Vec<T>, Box<dyn Error>> {
        if self.columns.is_empty() {
            return Err("No columns in the data structure!".into());
        }
        if self.columns[0].get(idx).is_none() {
            return Err("Row not in the table!".into());
        }
        Ok(self.columns.iter().filter_map(|c| c.get(idx)).cloned().collect())
    }
}
