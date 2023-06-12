// nominal.rs

/// This parses values in the feature column as nominal values. Takes each unique 
/// type of string and assigns them a value from 0 to 2^(N - 1). Where N is the 
/// number of unique strings in the feature column.

use super::Parser;
use crate::data::column::Column;
use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

pub struct NominalParser;

impl Parser for NominalParser {
    fn parse(
        column: &Column<Option<String>>,
    ) -> Result<Column<Option<Numeric>>, Box<dyn Error>> {
        // Create the returned column and a map to hold the unique values in the input
        let mut ret = Column::<Option<Numeric>>::new();
        let mut map = HashMap::<String, u32>::new();
        let mut next_bitshift = 0;

        // Iterate through all values in the colum and check the map is a coded value exists
        for value in column.values() {
            let parsed = value.as_ref().map(|value| {
                if let Some(found) = map.get(value) {
                    // Create a new value of the coded value in the column
                    Numeric::from(*found)
                } else {
                    // No coded value was found in the map, so we make a new one
                    let coding = 1 << next_bitshift;
                    map.insert(value.to_owned(), coding);
                    next_bitshift += 1;
                    Numeric::from(coding)
                }
            });
            ret.push(parsed);
        }

        // The original map serves to convert string -> value, we must invert this to get a map 
        // that converts values -> string
        let mut value_map = HashMap::<u32, String>::new();
        for (value, encoding) in map {
            value_map.insert(encoding, value);
        }

        // Set the metadata in the column to store the coding to string map
        ret.set_metadata(value_map);
    
        Ok(ret)
    }
}
