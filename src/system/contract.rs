use std::collections::HashMap;
use anyhow::Result;

pub trait EnvVars {
    fn prefixed(prefix: &str) -> Result<HashMap<String, String>>;
}

pub trait FileSystem {
    fn read_string(path: &str) -> Result<String>;
    fn write_string(path: &str, content: &str) -> Result<()>;
}

