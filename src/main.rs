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
use pipeline::models;
use pipeline::evaluation;

fn main() -> Result<(), Box<dyn Error>> {
    // Check if user gave command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("No command line arguments given!".into());
    }

    // Open file specified on the path given as argument in the command line
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
    let mut parsed = parsers::parse_input(input, configs.parsing)?;

    for col in parsed.columns() {
        println!("{}", col);
    }

    // Scrubbing stage, this stage replaces missing values and all missing
    // values are dealt with
    if let Some(configs) = configs.scrub {
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
    if let Some(configs) = configs.transform {
        transform::apply(&mut cleaned, configs)?;

        for col in cleaned.columns() {
            println!("{}", col);
        }
    }

    // Create a training data partitioner for cross-correlation validaton
    let partitioner = validation::get_partitioner(&configs.model.validation.strategy)?;
    let folds = partitioner(&cleaned, configs.model.label_index, configs.model.validation.parameters)?;

    // Fetch evaluator specified on configuration file
    let evaluator = evaluation::get_evaluator(&configs.model.evaluation)?;

    // Fetch the model specified on configuration file
    let mut model = models::get_model(&configs.model.name)?;

    let mut model_output = Vec::new();
    let mut validation_set = Vec::new();
    let mut training_set = Vec::new();
    for (fold_idx, (train_indices, validation_indices)) in folds.iter().enumerate() {
        println!("\nFOLD #: {}", fold_idx);

        // Create training data set
        println!("TRAINING");
        training_set.clear();
        for &idx in train_indices {
            training_set.push(cleaned.get_row(idx));
        }
        println!("SIZE: {}", training_set.len());

        // Train model on training data set
        model.train(&training_set, configs.model.label_index)?;

        // Use model to evaluate performance of training data
        model_output.clear();
        for sample in training_set.iter() {
            model_output.push(model.predict(sample)?);
        }

        // Calculate performance
        let training_performance = evaluator(&model_output, &training_set, configs.model.label_index)?;
        println!("SCORE: {}", training_performance);

        // Create validation data set
        println!("VALIDATION");
        validation_set.clear();
        for &idx in validation_indices {
            validation_set.push(cleaned.get_row(idx));
        }
        println!("SIZE: {}", validation_set.len());

        // Use model to predict labels on validation data
        model_output.clear();
        for sample in validation_set.iter() {
            model_output.push(model.predict(sample)?);
        }

        // Calculate validation performance
        let validation_performance = evaluator(&model_output, &validation_set, configs.model.label_index)?;
        println!("SCORE: {}", validation_performance);
    }

    Ok(())
}
