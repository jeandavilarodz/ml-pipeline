use pipeline::input;
use pipeline::parsers;
use pipeline::scrubbers;


fn main() {
    let input = input::with_headers("datasets/test.csv", "csv");
    if let Err(error) = input {
        println!("{}", error.to_string());
        return;
    }

    let input = input.unwrap();
    for col in input.columns() {
        println!("{}", col);
    }

    let parsed = parsers::parse_input(input, vec!["ordinal", "nominal", "numerical"], vec!["?"]).expect("Could not parse input");
    for col in parsed.columns() {
        println!("{:?}", col);
    }

    let cleaned = scrubbers::scrub(parsed, vec![("mean", 2)]);
    if let Err(error) = cleaned {
        println!("{}", error.to_string());
        return;
    }

    let cleaned = cleaned.unwrap();
    for col in cleaned.columns() {
        println!("{}", col);
    }


}
