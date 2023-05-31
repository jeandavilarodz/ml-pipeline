use pipeline::input;
use pipeline::parsers;


fn main() {
    let input = input::with_headers("datasets/test.csv", "csv");
    if let Ok(input) = input {
        for col in input.columns() {
            println!("{}", col);
        }
        let parsed = parsers::parse_input(input, vec!["ordinal", "nominal", "numerical"], vec!["?"]).expect("Could not parse input");
        for col in parsed.columns() {
            println!("{:#?}", col);
        }
    }
}
