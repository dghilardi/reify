use std::collections::HashMap;

use crate::system::contract::{EnvVars, FileSystem};

pub struct NativeSystem;

impl EnvVars for NativeSystem {
    fn prefixed(prefix: &str) -> anyhow::Result<HashMap<String, String>> {
        let result = envy::prefixed(prefix).from_env()?;
        Ok(result)
    }
}

impl FileSystem for NativeSystem {
    fn read_string(path: &str) -> anyhow::Result<String> {
        let content = std::fs::read_to_string(path)?;
        Ok(content)
    }

    fn write_string(path: &str, content: &str) -> anyhow::Result<()> {
        std::fs::write(path, content)?;
        Ok(())
    }
}