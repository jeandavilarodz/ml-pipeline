//! This section includes the parsing logic for data in table

mod nominal;
mod null;
mod numerical;
mod ordinal;

use crate::data::column::Column;
use crate::data::data_frame::DataFrame;
use crate::types::Numeric;

use std::collections::HashMap;
use std::error::Error;

use lazy_static::lazy_static;
use simple_error;

pub trait Parser {
    fn parse(&self, column: &Column<String>, missing_indicators: &Vec<&str>) -> Column<Option<Numeric>>;
}

lazy_static! {
    static ref PARSE_TYPE_MAP: HashMap<&'static str, Box<dyn Parser + Sync>> = HashMap::from([
        (
            "numerical",
            Box::new(numerical::NumericalParser) as Box<dyn Parser + Sync>
        ),
        (
            "nominal",
            Box::new(nominal::NominalParser) as Box<dyn Parser + Sync>
        ),
        (
            "ordinal",
            Box::new(ordinal::OrdinalParser) as Box<dyn Parser + Sync>
        ),
        ("null", Box::new(null::NullParser) as Box<dyn Parser + Sync>),
    ]);
}

pub fn parse_input(
    table: DataFrame<String>,
    parsers: Vec<&str>,
    missing_indicators: Vec<&str>,
) -> Result<DataFrame<Option<Numeric>>, Box<dyn Error>> {
    let mut ret = DataFrame::<Option<Numeric>>::new();
    if table.columns().len() != parsers.len() {
        simple_error::bail!("Did not provide enough parsers per table column!");
    }

    let missing_parser = parsers
        .iter()
        .fold(false, |acc, p| acc | !PARSE_TYPE_MAP.contains_key(p));

    if missing_parser {
        simple_error::bail!("Parser type not supported!");
    }

    let mut parsed_cols: Vec<Column<Option<Numeric>>> = Vec::new();
    for (parser, col) in parsers.iter().zip(table.columns()) {
        let parser = &PARSE_TYPE_MAP[parser];
        let mut new_col = parser.parse(col, &missing_indicators);
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