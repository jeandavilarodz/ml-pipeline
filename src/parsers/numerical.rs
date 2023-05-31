use crate::data::column::Column;
use crate::types::Numeric;
use super::Parser;


pub struct NumericalParser;

impl Parser for NumericalParser {
    fn parse(&self, column: &Column<Option<&str>>) -> Column<Option<Numeric>> {
        let mut ret = Column::<Option<Numeric>>::new();
        for value in column.values() {
            ret.push(value.map(|v| v.parse::<Numeric>().ok()));
        }
        ret
    }
}