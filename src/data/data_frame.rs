use crate::data::column::Column;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct DataFrame<T: Sized> {
    columns: Vec<Column<T>>,
    column_idx_map: HashMap<String, usize>,
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
            self.column_idx_map.insert(name.to_owned(), self.columns.len());
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
            _ => None
        }
    }

    pub fn get_column_name_mut(&mut self, name: &str) -> Option<&mut Column<T>> {
        match self.column_idx_map.get(name) {
            Some(idx) => self.columns.get_mut(*idx),
            _ => None
        }
    }
}

impl<T> fmt::Display for DataFrame<T>
where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num_rows = self.columns().last().unwrap().values().len();
        let num_cols = self.columns().len();
    
        for column in self.columns.iter() {
            write!(f, "|")?;
            match column.get_name() {
                Some(name) => write!(f, "{name:-^20}")?,
                _ => write!(f, " {:-^20} ", "N/A")?,
            }
        }
        write!(f, "|\n")?;



        let stop = num_rows.min(10);
        for idx in 0..stop-1 {
            for column in self.columns() {
                write!(f, "|")?;
                match column.get(idx) {
                    Some(val) => write!(f, "{val:<20}")?,
                    None => write!(f, "{:<20}", "None")?,
                }
            }
            write!(f, "|\n")?;
        }
        if num_rows > 10 {
            for _ in 0..num_cols {
                write!(f, "|{:^20}", "...")?;
            }
            write!(f, "|\n")?;
        }
        for column in self.columns() {
            let val = column.values().last().unwrap();
            write!(f, "|{val:<20}")?;
        }
        write!(f, "|\n")?;
        for _ in 0..num_cols {
            write!(f, "{:-<21}", "|")?;
        }
        write!(f, "|\n")?;
        Ok(())
    }
}
