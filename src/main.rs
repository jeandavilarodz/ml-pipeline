use pipeline::input;
use pipeline::parsers;
use pipeline::scrubbers;
use pipeline::transform;

fn main() {
    let input = input::read_input("datasets/test.csv", "csv", vec!["?"], true);
    if let Err(error) = input {
        println!("{}", error.to_string());
        return;
    }

    let input = input.unwrap();
    for col in input.columns() {
        println!("{:?}", col);
    }

    let parsed = parsers::parse_input(input, vec!["nominal", "ordinal", "numerical"])
        .expect("Could not parse input");
    for col in parsed.columns() {
        println!("{:?}", col);
    }

    let cleaned = scrubbers::scrub(parsed, vec![("mean", 2)]);
    if let Err(error) = cleaned {
        println!("{}", error.to_string());
        return;
    }

    let mut cleaned = cleaned.unwrap();
    for col in cleaned.columns() {
        println!("{}", col);
    }

    let result = transform::apply(&mut cleaned, vec![("normalize", 2)], None);
    if let Err(error) = result {
        println!("{}", error.to_string());
        return;
    }

    for col in cleaned.columns() {
        println!("{}", col);
    }
}
