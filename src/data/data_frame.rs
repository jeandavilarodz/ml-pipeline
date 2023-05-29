use crate::data::column::Column;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DataFrame<T: Sized> {
    columns: Vec<Box<Column<T>>>,
    column_idx_map: HashMap<String, usize>,
}

impl<T> DataFrame<T> {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            column_idx_map: HashMap::new(),
        }
    }
    
    pub fn add_column(&mut self, column: Column<T>) {
        if let Some(name) = column.get_name() {
            self.column_idx_map.insert(name.to_owned(), self.columns.len());
        }
        self.columns.push(Box::new(column));
    }
    
    pub fn get_column_idx(&self, idx: usize) -> Option<&Box<Column<T>>> {
        self.columns.get(idx)
    }
    
    pub fn get_column_name(&self, name: &str) -> Option<&Box<Column<T>>> {
        match self.column_idx_map.get(name) {
            Some(idx) => self.columns.get(*idx),
            _ => None
        }
    }
}