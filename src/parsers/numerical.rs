// numerical.rs

/// This is a parser that takes in the input string and tries to parse it as a number.
/// If the strings in the input column are not numeric then it places a missing value.

use super::Parser;
use crate::data::column::Column;
use crate::types::Numeric;

use std::error::Error;

pub struct NumericalParser;

impl Parser for NumericalParser {
    fn parse(
        column: &Column<Option<String>>,
    ) -> Result<Column<Option<Numeric>>, Box<dyn Error>> {
        let mut ret = Column::<Option<Numeric>>::new();
        for value in column.values() {
            ret.push(value.as_ref().and_then(|v| v.parse::<Numeric>().ok()));
        }
        Ok(ret)
    }
}
