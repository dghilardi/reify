use std::collections::HashMap;

use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ReifyConfig {
    pub env: HashMap<String, String>,
    pub mounts: Vec<ReifyMount>,
}

#[derive(Deserialize, Debug)]
pub struct ReifyMount {
    pub source: String,
    pub destination: String,
    pub processor: ReifyProcessor,
}

#[derive(Deserialize, Debug)]
pub enum ReifyProcessor {
    #[serde(rename = "handlebars")]
    Handlebars,
    #[serde(rename = "copy")]
    Copy,
}

pub fn parse_config(cfg_path: &str) -> Result<ReifyConfig, ConfigError> {
    let mut cfg = Config::new();
    cfg.merge(File::with_name(cfg_path).required(false))?;
    cfg.try_into()
}