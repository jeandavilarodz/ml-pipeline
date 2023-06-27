// kx2_folds.rs

//! This logic just creates a partition of the input DataFrame,
//! builds a model, trains and evaluates the model for each partition
//! using the kx2 cross-validation algorithm.

use crate::config::ConfigStruct;
use crate::data::data_frame::DataFrame;
use crate::types::Numeric;

use crate::evaluation;
use crate::models;
use crate::tuning;
use crate::validation;

use plotly::color::NamedColor;
use plotly::layout::ShapeLine;
use rand::prelude::*;
use rand::seq::SliceRandom;

use std::collections::HashMap;
use std::error::Error;

use plotly::common::{Marker, Mode, Title};
use plotly::layout::{Axis, Shape};
use plotly::ImageFormat;
use plotly::{Layout, Plot, Scatter};

const MAKE_PLOTS: bool = true;

pub fn train_and_evaluate(
    df: &DataFrame<Numeric>,
    configs: &ConfigStruct,
) -> Result<f64, Box<dyn Error>> {
    // Create a training data partitioner for cross-correlation validaton
    let partition = validation::get_partitioner(&configs.training.partitioning.strategy)?;

    // Fetch evaluator specified on configuration file
    let evaluate = evaluation::get_evaluator(&configs.training.evaluation)?;

    // Fetch the model specified on configuration file
    let mut model_builder = models::get_model_builder(&configs.training.model.name)?;

    // Split the training data into training and validation set
    let first_fold_config = HashMap::from([("num_folds".to_string(), 5.0)]);
    let first_folds = partition(df, configs.training.label_index, &first_fold_config)?;

    // Choose a random 80-20 split of the original data frame
    let (training_indexes, validation_indexes) =
        first_folds.choose(&mut rand::thread_rng()).unwrap();

    // Geenrate validation set (20% of the original data set)
    let mut validation_set = Vec::new();
    for &idx in validation_indexes.iter() {
        validation_set.push(df.get_row(idx)?.into_boxed_slice());
    }
    println!("validation_set.len(): {}", validation_set.len());

    // Generate a data frame for the training and testing set (80% of the original data)
    let mut training_and_testing_set = Vec::new();
    for &idx in training_indexes.iter() {
        training_and_testing_set.push(df.get_row(idx)?.into_boxed_slice());
    }
    println!(
        "training_and_testing_set.len(): {}",
        training_and_testing_set.len()
    );
    let training_and_testing_df = DataFrame::from_rows(training_and_testing_set)?;

    let mut first_set = Vec::new();
    let mut second_set = Vec::new();
    let mut model1_predictions = Vec::new();
    let mut model2_predictions = Vec::new();
    let mut models = Vec::new();
    let folding_config = HashMap::from([("num_folds".to_string(), 2.0)]);

    let hyperparameter_combinations =
        tuning::grid_search_tuning::get_hyperparameter_combinations(
            &configs.training.model.tunning,
        )?;

    for _ in 0..5 {
        let folds = partition(
            &training_and_testing_df,
            configs.training.label_index,
            &folding_config,
        )?;

        let (first_set_indexes, second_set_indexes) = &folds[0];

        // Create training set samples
        first_set.clear();
        for &idx in first_set_indexes.iter() {
            first_set.push(training_and_testing_df.get_row(idx)?.into_boxed_slice());
        }

        // Create testing set samples
        second_set.clear();
        for &idx in second_set_indexes.iter() {
            second_set.push(training_and_testing_df.get_row(idx)?.into_boxed_slice());
        }

        // Pluck two combinations from hyperparameter combinations
        let combination_idx = rand::thread_rng().gen_range(0..hyperparameter_combinations.len());
        let tuning_hyperparameter_1 = hyperparameter_combinations
            .get(combination_idx)
            .ok_or("Error fetching hyperparameter combination!")?;
        let combination_idx = rand::thread_rng().gen_range(0..hyperparameter_combinations.len());
        let tuning_hyperparameter_2 = hyperparameter_combinations
            .get(combination_idx)
            .ok_or("Error fetching hyperparameter combination!")?;

        // Create two model instances
        model_builder.with_hyperparameters(&tuning_hyperparameter_1)?;
        let model1 = model_builder.build(&first_set, configs.training.label_index)?;
        model_builder.with_hyperparameters(&tuning_hyperparameter_2)?;
        let model2 = model_builder.build(&second_set, configs.training.label_index)?;

        println!(
            "model 1 trying hyper parameter combination {:?}",
            &tuning_hyperparameter_1
        );
        println!(
            "model 2 trying hyper parameter combination {:?}",
            &tuning_hyperparameter_2
        );

        // Generate predictions for the first model
        model1_predictions.clear();
        for sample in validation_set.iter() {
            let res = match configs.training.model.task.as_str() {
                "regression" => Ok(model1.predict(sample)),
                "classification" => Ok(model1.label(sample)),
                _ => Err("Invalid model task, only regression and classification are supported"),
            }?;
            model1_predictions.push(res);
        }

        // Generate predictions for the second model
        model2_predictions.clear();
        for sample in validation_set.iter() {
            let res = match configs.training.model.task.as_str() {
                "regression" => Ok(model2.predict(sample)),
                "classification" => Ok(model2.label(sample)),
                _ => Err("Invalid model task, only regression and classification are supported"),
            }?;
            model2_predictions.push(res);
        }

        // Evaluate the first model
        let model1_error_metric = evaluate(
            &model1_predictions,
            &validation_set,
            configs.training.label_index,
        )?;

        // Evaluate the second model
        let model2_error_metric = evaluate(
            &model2_predictions,
            &validation_set,
            configs.training.label_index,
        )?;

        // Push model snapshots
        models.push((model1, model1_error_metric));
        models.push((model2, model2_error_metric));
    }

    // Print model hyperparameters with performance for debug
    for (model, error_metric) in models.iter() {
        println!(
            "error_metric: {}\nhyper-parameters:\n{:#?}",
            error_metric,
            model.get_hyperparameters()
        );
    }

    // Choose the model with best performance
    let (best_model, best_performance) = models
        .iter()
        .min_by(|(_, perf_1), (_, perf_2)| perf_1.abs().partial_cmp(&perf_2.abs()).unwrap())
        .expect("No best model found!");

    // Get the average performance of the model parameter tunning
    let avg_model_error_metric = models
        .iter()
        .fold(0.0, |acc, (_, model_error_metric)| acc + model_error_metric)
        / models.len() as f64;

    println!("Best model performance: {:?}", best_performance);
    println!(
        "Best model hyper-parameters:\n{:#?}",
        best_model.get_hyperparameters()
    );
    println!("Average model performance: {:?}", avg_model_error_metric);

    let best_hyperparameters = best_model.get_hyperparameters();
    let mut model_predictions = Vec::new();
    let mut model_error_metrics = Vec::new();
    let mut training_set = Vec::new();
    let mut testing_set = Vec::new();
    for _ in 0..5 {
        // This generates two folds
        let folds = partition(
            &training_and_testing_df,
            configs.training.label_index,
            &folding_config,
        )?;

        // Iterate through folds, each time generating a new model
        for (training_indexes, testing_indexes) in folds {
            // Create training set samples
            training_set.clear();
            for &idx in training_indexes.iter() {
                training_set.push(training_and_testing_df.get_row(idx)?.into_boxed_slice());
            }

            // Create testing set samples
            testing_set.clear();
            for &idx in testing_indexes.iter() {
                testing_set.push(training_and_testing_df.get_row(idx)?.into_boxed_slice());
            }

            // Create model instance
            model_builder.with_hyperparameters(&best_hyperparameters)?;
            let model = model_builder.build(&training_set, configs.training.label_index)?;

            // Generate predictions for the model
            model_predictions.clear();
            for sample in testing_set.iter() {
                let res = match configs.training.model.task.as_str() {
                    "regression" => Ok(model.predict(sample)),
                    "classification" => Ok(model.label(sample)),
                    _ => {
                        Err("Invalid model task, only regression and classification are supported")
                    }
                }?;
                model_predictions.push(res);
            }

            // Evaluate the model
            let model_error_metric = evaluate(
                &model_predictions,
                &testing_set,
                configs.training.label_index,
            )?;

            // Push model error metrics
            model_error_metrics.push(model_error_metric);
            println!("model metrics:\n{:#?}", model.get_hyperparameters());
        }
    }

    let average_error =
        model_error_metrics.iter().fold(0.0, |acc, m| acc + m) / (model_error_metrics.len() as f64);

    println!("Model error metrics: {:?}", model_error_metrics);
    println!("Average error: {}", average_error);

    if MAKE_PLOTS {
        make_lollipop(
            model_error_metrics,
            "Experiment VS Error Metric",
            "error_metric_vs_experiment.png",
        );
    }

    Ok(average_error)
}

fn make_lollipop(data: Vec<f64>, title: &str, filename: &str) {
    let mut plot = Plot::new();
    let mut layout = Layout::new()
        .title(Title::new(title))
        .x_axis(Axis::new().dtick(1.0));

    for (exp_num, metric) in data.iter().enumerate() {
        layout.add_shape(
            Shape::new()
                .shape_type(plotly::layout::ShapeType::Line)
                .x0(exp_num)
                .y0(0.0)
                .x1(exp_num)
                .y1(*metric)
                .line(
                    ShapeLine::new()
                        .color(NamedColor::Gray)
                        .dash(plotly::common::DashType::Dash),
                ),
        )
    }
    plot.set_layout(layout);

    let exp_num = (0..data.len()).map(|x| x as f64).collect::<Vec<f64>>();
    plot.add_trace(
        Scatter::new(exp_num, data)
            .mode(Mode::Markers)
            .marker(Marker::new().color("red").size(15)),
    );

    plot.write_image(filename, ImageFormat::PNG, 1024, 1024, 1.0);
}
