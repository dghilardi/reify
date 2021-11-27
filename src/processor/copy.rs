use super::ReifyProcessor;

pub struct CopyProcessor;

impl ReifyProcessor for CopyProcessor {
    fn render(&self, source: &str) -> anyhow::Result<String> {
        Ok(String::from(source))
    }
}