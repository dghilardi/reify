pub mod handlebars;
pub mod context;

pub trait ReifyProcessor {
    fn render(&self, source: &str) -> anyhow::Result<String>;
}