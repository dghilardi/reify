use clap::Parser;

/// Write configuration files from templates using env variables
#[derive(Parser, Debug)]
#[clap(name = "reify")]
pub struct ReifyOpts {
    /// Configuration file
    #[clap(short, long)]
    pub config_file: String,

    #[clap(short, long, default_value = "reify")]
    pub env_prefix: String,

    /// Templates
    #[clap(short, long)]
    pub templates: Vec<String>,
}
