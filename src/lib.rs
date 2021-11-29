use anyhow::Context;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
mod cli;
mod common;
mod processor;
mod wasm_utils;
pub mod engine;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct ReifyRunArgs {
    cli_args: Vec<String>
}

#[wasm_bindgen]
pub fn run(js_args: js_sys::Array) {
    let args: Vec<String> = js_args.iter()
        .filter_map(|el| el.as_string())
        .collect();

    let result = engine::run(args)
        .context("Error running reify");

    if let Err(e) = result {
        wasm_utils::log(&format!("Error happened: {:?}", e));
    }
}
