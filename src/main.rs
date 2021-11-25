use clap::Parser;
use config::{Config, ConfigError, File};

use crate::cli::ReifyOpts;
use common::config::ReifyConfig;

mod cli;
mod common;

fn parse_config(cfg_path: &str) -> Result<ReifyConfig, ConfigError> {
    let mut cfg = Config::new();
    cfg.merge(File::with_name(cfg_path).required(false))?;
    cfg.try_into()
}

fn main() {
    let opts = ReifyOpts::parse();
    let config = parse_config(&opts.config_file)
        .expect("Error parsing configuration file");
    println!("opts: {:?}", config);
}
