use clap::Parser;
use config::{Config, ConfigError, File};

use crate::cli::ReifyOpts;
use common::config::ReifyConfig;
use crate::common::config::ReifyProcessor::Handlebars;
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

    for mount in config.mounts {
        match mount.processor {
            Handlebars => engine::process_template(&mount.source, &mount.destination, HandlebarsProcessor::new(&opts.env_prefix).expect("Error building processor"))
        }.expect("Error processing template")
    }
}
