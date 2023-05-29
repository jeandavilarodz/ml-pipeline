use pipeline::input::csv::CsvReader;
use pipeline::input::Reader;
use pipeline::parsers::{ordinal::OrdinalParser, numerical::NumericalParser, parse_table};


fn main() {
    let input = CsvReader::with_headers("datasets/test.csv");
    if let Ok(input) = input {
        for col in input.columns() {
            println!("{}", col);
        }
        let parsed = parse_table(input, vec![Box::new(OrdinalParser), Box::new(OrdinalParser), Box::new(NumericalParser)]).expect("Could not parse input");
        for col in parsed.columns() {
            println!("{:#?}", col);
        }
    }
}
