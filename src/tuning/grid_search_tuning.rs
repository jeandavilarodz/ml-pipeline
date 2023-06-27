extern crate itertools;
use itertools::Itertools;
use crate::models::ModelBuilder;

use std::collections::HashMap;
use std::error::Error;

pub fn fit(
    tuning_parameters: HashMap<String, Vec<f64>>,
    model_builder: &mut Box<dyn ModelBuilder>,
//    _cross_validation: usize,
) -> Result<f64, Box<dyn Error>> {
    // Create combinations of hyperparameters
    let combinations = tuning_parameters.iter().map(|(_, range)| range.iter()).multi_cartesian_product();

    // Iterate over all combinations of hyperparameters
    for combination in combinations {
        println!("{:?}", combination);
        let mut hyperparameter_values = HashMap::new();
        for ((param, _), value) in tuning_parameters.iter().zip(combination) {
            hyperparameter_values.insert(param.clone(), value.to_string());
        }

        println!("{:?}", hyperparameter_values);

        model_builder.with_hyperparameters(&hyperparameter_values)?;

        // Print the hyperparameters and the corresponding accuracy
    }

    Ok(0.0)
}
