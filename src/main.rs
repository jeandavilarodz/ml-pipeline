use std::collections::HashMap;

use pipeline::input;
use pipeline::parsers;
use pipeline::scrubbers;
use pipeline::transform;
use pipeline::types::Numeric;
use pipeline::validation::kfold_stratified::StratifiedKFold;

fn main() {
    let input = input::read_input("datasets/car.data", "csv", vec!["none"], false);
    if let Err(error) = input {
        println!("{}", error.to_string());
        return;
    }

    let input = input.unwrap();
    for col in input.columns() {
        println!("{}", col);
    }

    let parsed = parsers::parse_input(
        input,
        vec![
            "ordinal", "ordinal", "ordinal", "ordinal", "ordinal", "ordinal", "ordinal",
        ],
    )
    .expect("Could not parse input");
    for col in parsed.columns() {
        println!("{}", col);
    }

    let cleaned = scrubbers::scrub(parsed, vec![]);
    if let Err(error) = cleaned {
        println!("{}", error.to_string());
        return;
    }

    let mut cleaned = cleaned.unwrap();
    for col in cleaned.columns() {
        println!("{}", col);
    }
    let params = HashMap::from([("num_bins", 2.0 as Numeric)]);
    let result = transform::apply(
        &mut cleaned,
        vec![("equal-width-discretization", 2)],
        Some(params),
    );
    if let Err(error) = result {
        println!("{}", error.to_string());
        return;
    }

    for col in cleaned.columns() {
        println!("{}", col);
    }

    let folds = StratifiedKFold::partition(&cleaned, 6, 10).unwrap();

    for (fold_idx, (train_indices, validation_indices)) in folds.iter().enumerate() {
        println!("FOLD #: {}", fold_idx);
        println!("TRAINING");
        for &idx in train_indices {
            print!("{:?}, ", cleaned.get_row(idx));
        }
        print!("\n");
        println!("TRAINING SIZE: {}", train_indices.len());

        println!("VALIDATION");
        for &idx in validation_indices {
            print!("{:?}, ", cleaned.get_row(idx));
        }
        print!("\n");
        println!("VALIDATION SIZE: {}", validation_indices.len());
    }
}
