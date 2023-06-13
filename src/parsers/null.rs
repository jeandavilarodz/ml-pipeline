// null.rs

/// This is a test parser that invalidates a column

use super::Parser;
use crate::data::column::Column;
use crate::types::Numeric;

use std::error::Error;

pub struct NullParser;

impl Parser for NullParser {
    fn parse(
        _column: &Column<Option<String>>,
    ) -> Result<Option<Column<Option<Numeric>>>, Box<dyn Error>> {
        Ok(None)
    }
}
