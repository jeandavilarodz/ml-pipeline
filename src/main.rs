// main.rs

use std::error::Error;
use std::fs::File;
use std::env;

use pipeline::config::ConfigStruct;
use pipeline::input;
use pipeline::parsers;
use pipeline::scrubbers;
use pipeline::transform;
use pipeline::validation::kfold_stratified::StratifiedKFold;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("No command line arguments given!".into());
    }

    let file = File::open(&args[1])?;
    let configs: ConfigStruct = serde_yaml::from_reader(file)?;

    println!("{:#?}", configs);

    let input = input::read_input(configs.input)?;

    for col in input.columns() {
        println!("{}", col);
    }

    let parsed = parsers::parse_input(
        input,
        configs.parsing,
    )?;

    for col in parsed.columns() {
        println!("{}", col);
    }

    let mut cleaned = scrubbers::scrub(
        parsed, 
        vec![]
    )?;

    for col in cleaned.columns() {
        println!("{}", col);
    }

    transform::apply(
        &mut cleaned,
        configs.transform,
    )?;

    for col in cleaned.columns() {
        println!("{}", col);
    }

    let folds = StratifiedKFold::partition(&cleaned, 2, 10).unwrap();

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
