extern crate itertools;
use itertools::Itertools;

use std::collections::HashMap;
use std::error::Error;

pub fn get_hyperparameter_combinations(
    tuning_parameters: &HashMap<String, Vec<f64>>,
) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    // Create combinations of hyperparameters
    let combinations = tuning_parameters.iter().map(|(_, range)| range.iter()).multi_cartesian_product();

    // Iterate over all combinations of hyperparameters and return a list of hyperparameters
    let mut hyperparameter_combinations = Vec::new();
    for combination in combinations {
        let mut hyperparameter_values = HashMap::new();
        for ((param, _), value) in tuning_parameters.iter().zip(combination) {
            hyperparameter_values.insert(param.clone(), value.to_string());
        }
        hyperparameter_combinations.push(hyperparameter_values)
    }

    Ok(hyperparameter_combinations)
}
