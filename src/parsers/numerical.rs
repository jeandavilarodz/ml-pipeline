use crate::data::column::Column;
use crate::types::Numeric;
use super::Parser;


pub struct NumericalParser;

impl Parser for NumericalParser {
    fn parse(&self, column: &Column<Option<String>>) -> Column<Option<Numeric>> {
        let mut ret = Column::<Option<Numeric>>::new();
        for value in column.values() {
            ret.push(value.as_ref().map(|v| v.parse::<Numeric>().ok()).flatten());
        }
        ret
    }
}