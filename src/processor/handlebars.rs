use handlebars::{Handlebars};
use crate::processor::context::ReifyContext;
use crate::processor::ReifyProcessor;

pub struct HandlebarsProcessor<'a, Ctx> {
    ctx: &'a Ctx,
    registry: Handlebars<'a>
}

impl <'a, Ctx> HandlebarsProcessor<'a, Ctx> {
    pub fn new(ctx: &'a Ctx) -> anyhow::Result<Self> {
        Ok(Self {
            ctx,
            registry: Handlebars::new()
        })
    }
}

impl <'a, Ctx: ReifyContext> ReifyProcessor for HandlebarsProcessor<'a, Ctx> {
    fn render(&self, source: &str) -> anyhow::Result<String> {
        let result = self.registry.render_template(source, self.ctx.read_ctx())?;
        Ok(result)
    }
}