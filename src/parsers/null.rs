// null.rs

/// This is a test parser that sets everything to none.

use super::Parser;
use crate::data::column::Column;
use crate::types::Numeric;

use std::error::Error;

pub struct NullParser;

impl Parser for NullParser {
    fn parse(
        column: &Column<Option<String>>,
    ) -> Result<Column<Option<Numeric>>, Box<dyn Error>> {
        let mut ret = Column::<Option<Numeric>>::new();
        for _value in column.values() {
            ret.push(None);
        }
        Ok(ret)
    }
}
