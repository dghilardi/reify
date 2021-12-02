use std::collections::HashMap;
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