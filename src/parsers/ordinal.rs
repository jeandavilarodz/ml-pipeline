// ordinal.rs

//! Encodes strings as numbers

use crate::data::column::Column;
use crate::types::Numeric;
use super::Parser;

use std::collections::HashMap;

pub struct OrdinalParser;

impl Parser for OrdinalParser {
    fn parse(&self, table: &Column<String>) -> Column<Option<Numeric>> {
        let mut ret = Column::<Option<Numeric>>::new();
        let mut map = Vec::<String>::new();
        for value in table.values() {
            // TODO: Add logic to skip missing data
            if let Some(found) = map.iter().position(|v| v == value) {
                ret.push(Some(found as Numeric));
            }
            else {
                ret.push(Some(map.len() as Numeric));
                map.push(value.to_owned());
            }
        }
        let mut value_map = HashMap::<usize, String>::new();
        for (encoded, value) in map.into_iter().enumerate() {
            value_map.insert(encoded, value);
        }
        ret.set_metadata(value_map);
        ret
    }
}