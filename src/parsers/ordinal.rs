// ordinal.rs

/// This parses values in the feature column as ordinal values. Takes each unique 
/// type of string and assigns them a value from 0 to (N - 1). Where N is the 
/// number of unique strings in the feature column.

use super::Parser;
use crate::data::column::Column;
use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

pub struct OrdinalParser;

impl Parser for OrdinalParser {
    fn parse(
        column: &Column<Option<String>>,
    ) -> Result<Option<Column<Option<Numeric>>>, Box<dyn Error>> {
        let mut ret = Column::<Option<Numeric>>::new();
        let mut map = Vec::<String>::new();

        // Iterate through each value in the column and push the coded value into the return
        for value in column.values() {
            let parsed = value.as_ref().map(|val| {
                if let Some(found) = map.iter().position(|v| v == val) {
                    // There is a value in the map that corresponds to the string
                    Numeric::from(found as u32)
                } else {
                    // No value found in the map that codes the string, so we make a new one
                    map.push(val.to_owned());
                    Numeric::from((map.len() - 1) as u32)
                }
            });
            ret.push(parsed);
        }

        // We have an array where the position in the array will give a coded value
        // we need to use the position in the array to generate a map from numeric -> string
        let mut value_map = HashMap::<u32, String>::new();
        for (encoding, value) in map.into_iter().enumerate() {
            value_map.insert(encoding as u32, value);
        }

        // Set the metadata in the return column to store the numeric -> string mapping
        ret.set_metadata(value_map);

        Ok(Some(ret))
    }
}
