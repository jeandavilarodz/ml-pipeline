use crate::data::column::Column;
use crate::types::Numeric;
use super::Parser;


pub struct NullParser;

impl Parser for NullParser {
    fn parse(&self, column: &Column<Option<String>>) -> Column<Option<Numeric>> {
        let mut ret = Column::<Option<Numeric>>::new();
        for _value in column.values() {
            ret.push(None);
        }
        ret
    }
}