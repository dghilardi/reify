use std::collections::HashMap;
use crate::error::ReifyError;
use crate::system::contract::{EnvVars, FileSystem};
use crate::wasm_utils;


pub struct NodeJsSystem;

impl EnvVars for NodeJsSystem {
    fn prefixed(prefix: &str) -> anyhow::Result<HashMap<String, String>> {
        let full_env: HashMap<String, String> = wasm_utils::process::ENV.into_serde()?;
        let filtered_env = full_env.into_iter()
            .filter_map(|(key, value)|
                key
                    .strip_prefix(prefix)
                    .map(|subkey| (subkey.to_string(), value))
            )
            .collect();
        Ok(filtered_env)
    }
}

impl FileSystem for NodeJsSystem {
    fn read_string(path: &str) -> anyhow::Result<String> {
        let content = wasm_utils::read_file(path, "utf8")
            .map_err(ReifyError::from)?;
        Ok(content)
    }

    fn write_string(path: &str, content: &str) -> anyhow::Result<()> {
        wasm_utils::write_file(path, content)
            .map_err(ReifyError::from)?;
        Ok(())
    }
}