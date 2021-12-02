use anyhow::Context;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use crate::system::NodeJsSystem;
use crate::wasm_utils::set_panic_hook;

mod cli;
mod processor;
mod wasm_utils;
pub mod engine;
pub mod conf;
pub mod system;
mod error;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct ReifyRunArgs {
    cli_args: Vec<String>
}

#[wasm_bindgen]
pub fn run(js_args: js_sys::Array) {
    set_panic_hook();

    let args: Vec<String> = js_args.iter()
        .filter_map(|el| el.as_string())
        .collect();

    let result = engine::run::<_, _, NodeJsSystem>(args)
        .context("Error running reify");

    if let Err(e) = result {
        wasm_utils::log(&format!("Error happened: {:?}", e));
    }
}
