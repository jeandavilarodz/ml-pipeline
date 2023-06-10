use crate::data::column::Column;
use crate::types::Numeric;
use super::Parser;

use std::error::Error;

pub struct NullParser;

impl Parser for NullParser {
    fn parse(&self, column: &Column<Option<String>>) -> Result<Column<Option<Numeric>>, Box<dyn Error>> {
        let mut ret = Column::<Option<Numeric>>::new();
        for _value in column.values() {
            ret.push(None);
        }
        Ok(ret)
    }
}