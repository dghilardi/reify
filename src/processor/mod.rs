pub mod handlebars;
pub mod context;
pub mod copy;

pub trait ReifyProcessor {
    fn render(&self, source: &str) -> anyhow::Result<String>;
}