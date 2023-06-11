// main.rs

use std::env;
use std::error::Error;
use std::fs::File;

use pipeline::config::ConfigStruct;
use pipeline::input;
use pipeline::parsers;
use pipeline::scrubbers;
use pipeline::transform;
use pipeline::validation;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("No command line arguments given!".into());
    }

    let file = File::open(&args[1])?;
    let configs: ConfigStruct = serde_yaml::from_reader(file)?;

    // Input processing stage, this should read a file and return a table of String
    let read = input::get_reader(&configs.input.format)?;
    let input = read(
        &configs.input.address,
        &configs.input.missing_values,
        configs.input.headers,
    )?;

    for col in input.columns() {
        println!("{}", col);
    }

    // Parsing stage, this should convert the present strings to numbers
    let parsed = parsers::parse_input(input, configs.parsing)?;

    for col in parsed.columns() {
        println!("{}", col);
    }

    // Scrubbing stage, this stage replaces missing values and all missing
    // values are dealt with
    let mut cleaned = scrubbers::scrub(parsed, configs.scrub)?;

    for col in cleaned.columns() {
        println!("{}", col);
    }

    // Transform stage, this stage performs operations to the numbers
    transform::apply(&mut cleaned, configs.transform)?;

    for col in cleaned.columns() {
        println!("{}", col);
    }

    let partitioner = validation::get_partitioner(&configs.model.validation.strategy)?;
    let folds = partitioner(&cleaned, configs.model.validation.parameters)?;

    for (fold_idx, (train_indices, validation_indices)) in folds.iter().enumerate() {
        println!("FOLD #: {}", fold_idx);
        println!("TRAINING");
        for &idx in train_indices {
            print!("{:?}, ", cleaned.get_row(idx));
        }
        println!();
        println!("TRAINING SIZE: {}", train_indices.len());

        println!("VALIDATION");
        for &idx in validation_indices {
            print!("{:?}, ", cleaned.get_row(idx));
        }
        println!();
        println!("VALIDATION SIZE: {}", validation_indices.len());
    }

    Ok(())
}
