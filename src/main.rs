use clap::Parser;
use config::{Config, ConfigError, File};

use crate::cli::ReifyOpts;
use common::config::ReifyConfig;
use crate::common::config::ReifyProcessor;
use crate::processor::context::EnvContext;
use crate::processor::copy::CopyProcessor;
use crate::processor::handlebars::HandlebarsProcessor;

mod cli;
mod common;
mod engine;
mod processor;

fn parse_config(cfg_path: &str) -> Result<ReifyConfig, ConfigError> {
    let mut cfg = Config::new();
    cfg.merge(File::with_name(cfg_path).required(false))?;
    cfg.try_into()
}

fn main() {
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
