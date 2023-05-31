// numerical.rs

use crate::data::column::Column;
use crate::types::Numeric;
use super::Parser;

use std::collections::HashMap;

pub struct NominalParser;

impl Parser for NominalParser {
    fn parse(&self, column: &Column<Option<&str>>) -> Column<Option<Numeric>> {
        let mut ret = Column::<Option<Numeric>>::new();
        let mut map = HashMap::<String, usize>::new();
        let mut next_bitshift = 0;
        for value in column.values() {
            match value {
                None => ret.push(None),
                Some(found) => {
                    if let Some(found) = map.get(value) {
                        ret.push(Some(*found as Numeric));
                    }
                    else {
                        let coding = 1 << next_bitshift;
                        ret.push(Some(coding as Numeric));
                        map.insert(value.to_owned(), coding);
                        next_bitshift += 1;
                    }
                }
            }
        }
        let mut value_map = HashMap::<usize, String>::new();
        for (value, encoded) in map {
            value_map.insert(encoded, value);
        }
        ret.set_metadata(value_map);
        ret
    }
}