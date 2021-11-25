use std::collections::HashMap;
use std::env;
use handlebars::{Handlebars};
use crate::processor::ReifyProcessor;

pub struct HandlebarsProcessor<'a> {
    registry: Handlebars<'a>
}

impl <'a> HandlebarsProcessor<'a> {
    pub fn new() -> Self {
        Self {
            registry: Handlebars::new()
        }
    }
}

impl <'a> ReifyProcessor for HandlebarsProcessor<'a> {
    fn render(&self, source: &str) -> anyhow::Result<String> {
        let vars: HashMap<String, String> = envy::prefixed("reify_").from_env()?;
        let result = self.registry.render_template(source, &vars)?;
        Ok(result)
    }
}