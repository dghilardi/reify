use clap::Parser;

/// Write configuration files from templates using env variables
#[derive(Parser, Debug)]
#[clap(name = "reify")]
pub struct ReifyOpts {
    /// Configuration file
    #[clap(short, long)]
    pub config_file: String,

    /// Templates
    #[clap(short, long, multiple_occurrences = true)]
    pub templates: Vec<String>,
}