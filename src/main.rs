// main.rs

use std::env;
use std::error::Error;
use std::fs::File;

use pipeline::config::ConfigStruct;
use pipeline::input;
use pipeline::parsers;
use pipeline::scrubbers;
use pipeline::transform;
use pipeline::trainers;

fn main() -> Result<(), Box<dyn Error>> {
    // Check if user gave command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("No command line arguments given!".into());
    }

    // Open file specified on the path given as argument in the command line
    let file = File::open(&args[1])?;
    let configs: ConfigStruct = serde_yaml::from_reader(file)?;

    println!("######################################");
    println!("#############   INPUT   ##############");
    println!("######################################");

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
    println!("######################################");
    println!("#############  PARSING  ##############");
    println!("######################################");

    // Parsing stage, this should convert the present strings to numbers
    let mut parsed = parsers::parse_input(input, &configs.parsing)?;

    for col in parsed.columns() {
        println!("{}", col);
    }

    println!("######################################");
    println!("############# SCRUBBING ##############");
    println!("######################################");

    // Scrubbing stage, this stage replaces missing values and all missing
    // values are dealt with
    if let Some(configs) = configs.scrub.as_ref() {
        // There was a scrub stage specified in the configuration file, iterate through each scrubber
        // and clean features accordingly
        for config in configs {
            let scrubber = scrubbers::get_scrubber(&config.name)?;
            if let Some(column) = parsed.get_column_idx_mut(config.index) {
                scrubber(column)?;
            }
        }
    }

    // Amputate null values to turn Option<Numeric> to Numeric
    let mut cleaned = scrubbers::amputate(parsed)?;
    for col in cleaned.columns() {
        println!("{}", col);
    }

    // Transform stage, this stage performs operations to the numbers
    if let Some(configs) = configs.transform.as_ref() {
        println!("######################################");
        println!("############# TRANSFORM ##############");
        println!("######################################");
        transform::apply(&mut cleaned, configs)?;

        for col in cleaned.columns() {
            println!("{}", col);
        }
    }

    println!("######################################");
    println!("############# TRAINING  ##############");
    println!("######################################");

    trainers::train_and_evaluate(&cleaned, &configs)?;

    Ok(())
}
