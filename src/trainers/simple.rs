// simple.rs

//! This logic just creates a partition of the input DataFrame,
//! builds a model, trains and evaluates the model for each partition.

use crate::data::data_frame::DataFrame;
use crate::types::Numeric;
use crate::config::ConfigStruct;

use crate::validation;
use crate::evaluation;
use crate::models;

use std::error::Error;

pub fn train_and_evaluate(df: &DataFrame<Numeric>, configs: &ConfigStruct) -> Result<(), Box<dyn Error>>{
    // Create a training data partitioner for cross-correlation validaton
    let partitioner = validation::get_partitioner(&configs.training.partitioning.strategy)?;
    let folds = partitioner(
        df,
        configs.training.label_index,
        &configs.training.partitioning.parameters,
    )?;

    // Fetch evaluator specified on configuration file
    let evaluator = evaluation::get_evaluator(&configs.training.evaluation)?;

    // Fetch the model specified on configuration file
    let mut model_builder = models::get_model_builder(&configs.training.model.name)?;
    model_builder.with_parameters(&configs.training.model.parameters)?;

    let mut model_output = Vec::new();
    let mut validation_set = Vec::new();
    let mut training_set = Vec::new();
    for (fold_idx, (train_indices, validation_indices)) in folds.iter().enumerate() {
        println!("\nFOLD #: {}", fold_idx);

        // Create training data set
        println!("TRAINING");
        training_set.clear();
        for &idx in train_indices {
            training_set.push(df.get_row(idx)?.into_boxed_slice());
        }
        println!("SIZE: {}", training_set.len());

        // Train model on training data set
        let model = model_builder.build(&training_set, configs.training.label_index)?;

        // Use model to evaluate performance of training data
        model_output.clear();
        for sample in training_set.iter() {
            model_output.push(model.predict(sample));
        }

        // Calculate performance
        let training_performance =
            evaluator(&model_output, &training_set, configs.training.label_index)?;
        println!("ERROR: {}", training_performance);

        // Create validation data set
        println!("VALIDATION");
        validation_set.clear();
        for &idx in validation_indices {
            validation_set.push(df.get_row(idx)?.into_boxed_slice());
        }
        println!("SIZE: {}", validation_set.len());

        // Use model to predict labels on validation data
        model_output.clear();
        for sample in validation_set.iter() {
            model_output.push(model.predict(sample));
        }

        // Calculate validation performance
        let validation_performance =
            evaluator(&model_output, &validation_set, configs.training.label_index)?;
        println!("ERROR: {}", validation_performance);
    }

    Ok(())
}