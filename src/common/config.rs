use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ReifyConfig {
    pub mounts: Vec<ReifyMount>
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
}