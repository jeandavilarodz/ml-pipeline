// ordinal.rs

use super::Parser;
use crate::data::column::Column;
use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

pub struct OrdinalParser;

impl Parser for OrdinalParser {
    fn parse(
        &self,
        column: &Column<Option<String>>,
    ) -> Result<Column<Option<Numeric>>, Box<dyn Error>> {
        let mut ret = Column::<Option<Numeric>>::new();
        let mut map = Vec::<String>::new();
        for value in column.values() {
            let parsed = value
                .as_ref()
                .map(|val| {
                    if let Some(found) = map.iter().position(|v| v == val) {
                        Numeric::from(found as u32)
                    } else {
                        map.push(val.to_owned());
                        Numeric::from((map.len() - 1) as u32)
                    }
                });
            ret.push(parsed);
        }
        let mut value_map = HashMap::<u32, String>::new();
        for (encoded, value) in map.into_iter().enumerate() {
            value_map.insert(encoded as u32, value);
        }
        ret.set_metadata(value_map);
        Ok(ret)
    }
}
