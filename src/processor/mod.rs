pub mod handlebars;

pub trait ReifyProcessor {
    fn render(&self, source: &str) -> anyhow::Result<String>;
}