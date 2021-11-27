use clap::Parser;
use config::{Config, ConfigError, File};
use wasm_bindgen::prelude::*;

use common::config::ReifyConfig;

use crate::cli::ReifyOpts;
use crate::common::config::{parse_config, ReifyProcessor};
use crate::processor::context::EnvContext;
use crate::processor::copy::CopyProcessor;
use crate::processor::handlebars::HandlebarsProcessor;

mod cli;
mod common;
mod engine;
mod processor;

#[wasm_bindgen]
pub fn run() {
    let opts = ReifyOpts::parse();
    let config = parse_config(&opts.config_file)
        .expect("Error parsing configuration file");

    let context = EnvContext::merge_default(config.env, &opts.env_prefix)
        .expect("Error loading configuration");

    for mount in config.mounts {
        match mount.processor {
            ReifyProcessor::Handlebars => engine::process_template(&mount.source, &mount.destination, HandlebarsProcessor::new(&context).expect("Error building processor")),
            ReifyProcessor::Copy => engine::process_template(&mount.source, &mount.destination, CopyProcessor),
        }.expect("Error processing template")
    }
}
