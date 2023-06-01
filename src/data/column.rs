use std::collections::HashMap;
use std::fmt;

const DISPLAY_MAX: usize = 5;

#[derive(Debug)]
pub struct Column <T: Sized>{
    name: Option<String>,
    metadata: Option<HashMap<usize, String>>,
    values: Vec<T>,
}

impl <T>Column<T> {
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
        return match &self.name {
            Some(name) => Some(&name),
            None => None
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
    
    pub fn set_metadata(&mut self, metadata: HashMap<usize, String>) {
        self.metadata = Some(metadata);
    }

    pub fn get_metadata(&self) -> Option<&HashMap<usize, String>> {
        return match &self.metadata {
            Some(metadata) => Some(&metadata),
            None => None
        }
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
where T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.name {
            Some(name) => write!(f, "{:-^16}\n", name)?,
            _ => write!(f, "{:-^16}\n", "N/A")?,
        }
        if self.values.len() <= DISPLAY_MAX {
            for val in self.values.iter() {
                write!(f, "{:?}\n", val)?;
            }
            return Ok(())
        }
        for val in self.values[0..(DISPLAY_MAX - 1)].iter() {
            write!(f, "{:?}\n", val)?;
        }
        write!(f, "{}\n", "...")?;
        write!(f, "{:?}\n", self.values.last().unwrap())?;
        return Ok(())
    }
}
