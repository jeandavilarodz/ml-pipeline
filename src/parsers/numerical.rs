use crate::data::column::Column;
use crate::types::Numeric;
use super::Parser;


pub struct NumericalParser;

impl Parser for NumericalParser {
    fn parse(&self, column: &Column<String>, missing_indicators: &Vec<&str>) -> Column<Option<Numeric>> {
        let mut ret = Column::<Option<Numeric>>::new();
        for value in column.values() {
            if missing_indicators.contains(&value.as_str()) {
                ret.push(None);
                continue;
            }
            ret.push(value.parse::<Numeric>().ok());
        }
        ret
    }
}