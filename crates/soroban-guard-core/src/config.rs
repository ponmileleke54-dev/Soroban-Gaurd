use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConfigSeverity {
    Critical,
    Warning,
    Info,
    Ignore,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GuardConfig {
    #[serde(default)]
    pub rules: HashMap<String, ConfigSeverity>,
    #[serde(default)]
    pub exclude: Vec<String>,
}

impl GuardConfig {
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Self {
        if let Ok(content) = fs::read_to_string(path) {
            toml::from_str(&content).unwrap_or_default()
        } else {
            GuardConfig::default()
        }
    }
}
