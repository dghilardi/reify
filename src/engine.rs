use std::ffi::OsString;
use anyhow::Context;
use crate::cli::ReifyOpts;
use crate::common::config::{parse_config, ReifyProcessor};
use crate::processor::context::EnvContext;
use crate::processor::copy::CopyProcessor;
use crate::processor::handlebars::HandlebarsProcessor;
use clap::Parser;
pub fn process_template<P: crate::processor::ReifyProcessor>(src_path: &str, dst_path: &str, processor: P) -> anyhow::Result<()> {
    let template = std::fs::read_to_string(src_path)?;
    let rendered = processor.render(&template)?;
    std::fs::write(dst_path, rendered)?;
    Ok(())
}

pub fn run<I, T>(args: I) -> anyhow::Result<()>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
{
    let opts = ReifyOpts::try_parse_from(args)
        .context("Error parsing args")?;
    let config = parse_config(&opts.config_file)
        .context("Error parsing configuration file")?;

    let context = EnvContext::merge_default(config.env, &opts.env_prefix)
        .context("Error loading configuration")?;

    for mount in config.mounts {
        match mount.processor {
            ReifyProcessor::Handlebars => process_template(&mount.source, &mount.destination, HandlebarsProcessor::new(&context).context("Error building processor")?),
            ReifyProcessor::Copy => process_template(&mount.source, &mount.destination, CopyProcessor),
        }.context("Error processing template")?
    }

    Ok(())
}