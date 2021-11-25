use clap::Parser;

/// Write configuration files from templates using env variables
#[derive(Parser, Debug)]
#[clap(name = "reify")]
pub struct ReifyOpts {
    /// Configuration file
    #[clap(short, long)]
    config_file: Option<String>,

    /// Templates
    #[clap(short, long, multiple_occurrences = true)]
    templates: Vec<String>,
}