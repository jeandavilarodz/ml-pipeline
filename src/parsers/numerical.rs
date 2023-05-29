use crate::data::column::Column;
use crate::types::Numeric;
use super::Parser;


pub struct NumericalParser;

impl Parser for NumericalParser {
    fn parse(&self, table: &Column<String>) -> Column<Option<Numeric>> {
        let mut ret = Column::<Option<Numeric>>::new();
        for value in table.values() {
            ret.push(value.parse::<Numeric>().ok());
        }
        ret
    }
}