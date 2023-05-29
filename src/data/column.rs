use std::collections::HashMap;
use std::fmt;

const DISPLAY_MAX: usize = 5;

#[derive(Debug)]
pub struct Column <T: Sized>{
    name: Option<String>,
    metadata: HashMap<usize, String>,
    values: Vec<T>,
}

impl <T>Column<T> {
    pub fn new() -> Self {
        Self {
            name: None,
            metadata: HashMap::new(),
            values: Vec::new(),
        }
    }
    
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }
    
    pub fn append(&mut self, values: Vec<T>) {
        self.values.extend(values);
    }
    
    pub fn push(&mut self, value: T) {
        self.values.push(value);
    }
    
    pub fn get_metadata(&self) -> &HashMap<usize, String> {
        &self.metadata
    }
}

impl<T> fmt::Display for Column<T>
where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.name {
            Some(name) => write!(f, "{:-^16}\n", name)?,
            _ => write!(f, "{:-^16}\n", "N/A")?,
        }
        if self.values.len() <= DISPLAY_MAX {
            for val in self.values.iter() {
                write!(f, "{:^16}\n", val)?;   
            }
            return Ok(())
        }
        for val in self.values[0..(DISPLAY_MAX - 1)].iter() {
            write!(f, "{:^16}\n", val)?;   
        }
        write!(f, "{:^16}\n", "...")?;
        write!(f, "{:^16}\n", self.values.last().unwrap())?;
        return Ok(())
    }
}
