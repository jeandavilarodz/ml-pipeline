//! This section includes the parsing logic for data in table

pub mod numerical;
pub mod null;

use crate::data::data_frame::DataFrame;
use crate::data::column::Column;
use crate::types::Numeric;

use std::error::Error;
use simple_error;


pub trait Parser {
    fn parse(&self, table: &Column<String>) -> Column<Option<Numeric>>;
}

pub fn parse_table(table: DataFrame<String>, parsers: Vec<Box<dyn Parser>>) -> Result<DataFrame<Option<Numeric>>, Box<dyn Error>> {
    let mut ret = DataFrame::<Option<Numeric>>::new();
    if table.columns().len() != parsers.len() {
        simple_error::bail!("Did not provide enough parsers per table column!");
    }
    

    let mut parsed_cols: Vec<Column<Option<Numeric>>> = Vec::new();
    for (parser, col) in parsers.iter().zip(table.columns()) {
        let mut new_col = parser.parse(col);
        if let Some(name) = col.get_name() {
            new_col.set_name(name.to_owned());
        }
        if let Some(metadata) = col.get_metadata() {
            new_col.set_metadata(metadata.to_owned());
        }
        parsed_cols.push(new_col);
    }
    parsed_cols.into_iter().for_each(|col| ret.add_column(col));
    
    Ok(ret)
}