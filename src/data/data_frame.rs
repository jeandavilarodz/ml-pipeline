use itertools::Itertools;

use crate::data::column::Column;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DataFrame<T: Sized> {
    columns: Vec<Column<T>>,
    column_idx_map: HashMap<String, usize>,
}

impl<T> Default for DataFrame<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DataFrame<T> {
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

    pub fn get_row(&self, idx: usize) -> Vec<&T> {
        self.columns.iter().filter_map(|c| c.get(idx)).collect_vec()
    }
}
