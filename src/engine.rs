use std::ffi::OsString;
use anyhow::Context;
use crate::cli::ReifyOpts;
use crate::processor::copy::CopyProcessor;
use crate::processor::handlebars::HandlebarsProcessor;
use clap::Parser;
use crate::conf::loader::ConfigLoader;
use crate::conf::model::ReifyProcessor;
use crate::processor::context::EnvContext;
use crate::system::{EnvVars, FileSystem};

pub fn process_template<P: crate::processor::ReifyProcessor, S: FileSystem>(src_path: &str, dst_path: &str, processor: P) -> anyhow::Result<()> {
    let template = S::read_string(src_path)?;
    let rendered = processor.render(&template)?;
    S::write_string(dst_path, &rendered)?;
    Ok(())
}

pub fn run<I, T, S>(args: I) -> anyhow::Result<()>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
        S: EnvVars + FileSystem,
{
    let opts = ReifyOpts::try_parse_from(args)
        .context("Error parsing args")?;
    let config = ConfigLoader::parse_config::<S>(&opts.config_file)
        .context("Error parsing configuration file")?;

    let context = EnvContext::merge_default::<S>(config.env, &opts.env_prefix)
        .context("Error loading configuration")?;

    for mount in config.mounts {
        match mount.processor {
            ReifyProcessor::Handlebars => process_template::<_, S>(&mount.source, &mount.destination, HandlebarsProcessor::new(&context).context("Error building processor")?),
            ReifyProcessor::Copy => process_template::<_, S>(&mount.source, &mount.destination, CopyProcessor),
        }.context("Error processing template")?
    }

    Ok(())
}