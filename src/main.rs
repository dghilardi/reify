use clap::Parser;

use crate::cli::ReifyOpts;

mod cli;

fn main() {
    let opts = ReifyOpts::parse();
    println!("opts: {:?}", opts);
}
