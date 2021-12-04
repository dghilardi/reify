pub mod handlebars;
pub mod copy;
pub mod context;
pub mod tera;

pub trait ReifyProcessor {
    fn render(&self, source: &str) -> anyhow::Result<String>;
}