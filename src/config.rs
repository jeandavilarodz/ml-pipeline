// config.rs

/// This file specifies the structures inside the configuation files. It is used by the de-serialization
/// library in order to populate user values.

use std::collections::HashMap;
use serde::Deserialize;

// Structure defining fields in the validation sub-field of model stage
#[derive(Debug, Deserialize)]
pub struct ValidationConfigs {
    pub strategy: String,
    pub parameters: HashMap<String, f64>,
}

#[derive(Debug, Deserialize)]
pub struct ModelConfigs {
    pub name: String,
    pub task: String,
    pub parameters: Option<HashMap<String, f64>>,
}

#[derive(Debug, Deserialize)]
pub struct TunningConfigs {
    pub parameters: HashMap<String, Vec<f64>>,
}

// Structure defining fields in the mandatory model stage
#[derive(Debug, Deserialize)]
pub struct TrainingConfigs {
    pub model: ModelConfigs,
    pub evaluation: String,
    pub partitioning: ValidationConfigs,
    pub strategy: String,
    pub label_index: usize,
    pub tunning: TunningConfigs,
}

// Structure defining fields in the mandatory input stage
#[derive(Debug, Deserialize)]
pub struct InputStageConfigs {
    pub address: String,
    pub format: String,
    pub missing_values: Vec<String>,
    pub headers: bool,
}

// Structure defining fields in the scrubbing stage
#[derive(Debug, Deserialize)]
pub struct ScrubbingStageConfigs {
    pub name: String,
    pub index: usize,
}

// Structure defining fields in the transform stage
#[derive(Debug, Deserialize)]
pub struct TransformStageConfigs {
    pub name: String,
    pub index: usize,
    pub parameters: Option<HashMap<String, f64>>
}

// Overall structure defining the configuration stages
#[derive(Debug, Deserialize)]
pub struct ConfigStruct {
    pub input: InputStageConfigs,
    pub parsing: Vec<String>,
    pub scrub: Option<Vec<ScrubbingStageConfigs>>,
    pub transform: Option<Vec<TransformStageConfigs>>,
    pub training: TrainingConfigs,
}
