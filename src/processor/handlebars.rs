use std::collections::HashMap;
use handlebars::{Handlebars};
use crate::processor::ReifyProcessor;

pub struct HandlebarsProcessor<'a> {
    env: HashMap<String, String>,
    registry: Handlebars<'a>
}

impl <'a> HandlebarsProcessor<'a> {
    pub fn new(prefix: &str) -> anyhow::Result<Self> {
        Ok(Self {
            env: if prefix.ends_with('_') {
                envy::prefixed(prefix).from_env()?
            } else {
                envy::prefixed(format!("{}_", prefix)).from_env()?
            },
            registry: Handlebars::new()
        })
    }
}

impl <'a> ReifyProcessor for HandlebarsProcessor<'a> {
    fn render(&self, source: &str) -> anyhow::Result<String> {
        let result = self.registry.render_template(source, &self.env)?;
        Ok(result)
    }
}