use config::{Config, File, FileFormat};
use crate::conf::model::ReifyConfig;
use crate::system::FileSystem;
pub struct ConfigLoader;

impl ConfigLoader {
    pub fn parse_config<S: FileSystem>(cfg_path: &str) -> anyhow::Result<ReifyConfig> {
        let content = S::read_string(cfg_path)?;
        let mut cfg = Config::new();
        cfg.merge(File::from_str(&content, FileFormat::Toml))?;
        let parsed = cfg.try_into()?;
        Ok(parsed)
    }
}