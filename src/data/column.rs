// column.rs

/// This contains the abstraction for a column in a table data structure. It consists of a
/// dynamic array where values are kept, a column name (header) and some metadata about the
/// values in the column used for the mapping between numeric value and ordinal/nominal
/// data. It provides iterators so that algorithms can iterate through the values inside the
/// column. It also has abstractions to manipulate and retrieve individual elements in a
/// cell.

use std::collections::HashMap;
use std::fmt;

const DISPLAY_MAX: usize = 10;

#[derive(Debug)]
pub struct Column<T: Sized> {
    name: Option<String>,
    metadata: Option<HashMap<u32, String>>,
    values: Vec<T>,
}

impl<T> Default for Column<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Column<T> {
    pub fn new() -> Self {
        Self {
            name: None,
            metadata: None,
            values: Vec::new(),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn get_name(&self) -> Option<&str> {
        match &self.name {
            Some(name) => Some(name),
            None => None,
        }
    }

    pub fn append(&mut self, values: Vec<T>) {
        self.values.extend(values);
    }

    pub fn push(&mut self, value: T) {
        self.values.push(value);
    }

    pub fn remove(&mut self, idx: usize) {
        if idx >= self.values.len() {
            return;
        }
        self.values.remove(idx);
    }

    pub fn set_metadata(&mut self, metadata: HashMap<u32, String>) {
        self.metadata = Some(metadata);
    }

    pub fn get_metadata(&self) -> Option<&HashMap<u32, String>> {
        match &self.metadata {
            Some(metadata) => Some(metadata),
            None => None,
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.values.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.values.get_mut(index)
    }

    pub fn values(&self) -> std::slice::Iter<'_, T> {
        self.values.iter()
    }

    pub fn values_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.values.iter_mut()
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }
}

impl<T> fmt::Display for Column<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.name {
            Some(name) => writeln!(f, "{:-^16}", name)?,
            _ => writeln!(f, "{:-^16}", "N/A")?,
        }
        if self.values.len() <= DISPLAY_MAX {
            for val in self.values.iter() {
                writeln!(f, "{:?}", val)?;
            }
            return Ok(());
        }
        for val in self.values[0..(DISPLAY_MAX - 1)].iter() {
            writeln!(f, "{:?}", val)?;
        }
        writeln!(f, "...")?;
        writeln!(f, "{:?}", self.values.last().unwrap())?;
        Ok(())
    }
}
