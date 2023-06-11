// config.rs

use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ValidationConfigs {
    pub strategy: String,
    pub parameters: HashMap<String, f64>,
}

#[derive(Debug, Deserialize)]
pub struct ModelConfigs {
    pub name: String,
    pub label_index: usize,
    pub evaluation: String,
    pub validation: ValidationConfigs,
}

#[derive(Debug, Deserialize)]
pub struct InputStageConfigs {
    pub address: String,
    pub format: String,
    pub missing_values: Vec<String>,
    pub headers: bool,
}

#[derive(Debug, Deserialize)]
pub struct ScrubbingStageConfigs {
    pub name: String,
    pub index: usize,
}

#[derive(Debug, Deserialize)]
pub struct TransformStageConfigs {
    pub name: String,
    pub index: usize,
    pub parameters: Option<HashMap<String, f64>>
}

#[derive(Debug, Deserialize)]
pub struct ConfigStruct {
    pub input: InputStageConfigs,
    pub parsing: Vec<String>,
    pub scrub: Option<Vec<ScrubbingStageConfigs>>,
    pub transform: Option<Vec<TransformStageConfigs>>,
    pub model: ModelConfigs,
}
