// ordinal.rs

//! Encodes strings as numbers

use crate::data::column::Column;
use crate::types::Numeric;
use super::Parser;

use std::collections::HashMap;

pub struct OrdinalParser;

impl Parser for OrdinalParser {
    fn parse(&self, column: &Column<Option<String>>) -> Column<Option<Numeric>> {
        let mut ret = Column::<Option<Numeric>>::new();
        let mut map = Vec::<String>::new();
        for value in column.values() {
            let parsed = value.as_ref().map(|val| {
                if let Some(found) = map.iter().position(|v| v == val) {
                    found as Numeric
                }
                else {
                    map.push(val.to_owned());
                    (map.len() - 1) as Numeric
                }
            });
            ret.push(parsed);
        }
        let mut value_map = HashMap::<usize, String>::new();
        for (encoded, value) in map.into_iter().enumerate() {
            value_map.insert(encoded, value);
        }
        ret.set_metadata(value_map);
        ret
    }
}