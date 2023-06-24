//! This section includes the parsing logic for data in table. Only numerical, ordinal and nominal
//! data can be specified.

mod nominal;
mod null;
mod numerical;
mod ordinal;

use crate::data::column::Column;
use crate::data::data_frame::DataFrame;
use crate::types::Numeric;

use std::error::Error;

pub trait Parser {
    fn parse(
        column: &Column<Option<String>>,
    ) -> Result<Option<Column<Option<Numeric>>>, Box<dyn Error>>;
}

type ParseFnPtr = fn(&Column<Option<String>>) -> Result<Option<Column<Option<Numeric>>>, Box<dyn Error>>;

pub fn get_parser(name: &str) -> Result<ParseFnPtr, Box<dyn Error>> {
    match name {
        "numeric" => Ok(numerical::NumericalParser::parse),
        "nominal" => Ok(nominal::NominalParser::parse),
        "ordinal" => Ok(ordinal::OrdinalParser::parse),
        "null" => Ok(null::NullParser::parse),
        _ => Err("Invalid parser name given!".into()),
    }
}

pub fn parse_input(
    table: DataFrame<Option<String>>,
    parsers: &Vec<String>,
) -> Result<DataFrame<Option<Numeric>>, Box<dyn Error>> {
    let mut ret = DataFrame::<Option<Numeric>>::new();
    if table.columns().len() != parsers.len() {
        return Err("Did not provide enough parsers per table column!".into());
    }

    let mut parsed_cols: Vec<Column<Option<Numeric>>> = Vec::new();
    for (parser, col) in parsers.iter().zip(table.columns()) {
        let parser = get_parser(parser.as_str())?;
        let new_col = parser(col)?;
        if new_col.is_none() {
            continue;
        }
        let mut new_col = new_col.unwrap();
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
