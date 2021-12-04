use tera::{Context, Tera, to_value};
use crate::processor::context::ReifyContext;
use crate::processor::ReifyProcessor;

pub struct TeraProcessor<'a, Ctx> {
    ctx: &'a Ctx,
}

impl <'a, Ctx> TeraProcessor<'a, Ctx> {
    pub fn new(ctx: &'a Ctx) -> Self {
        Self {
            ctx,
        }
    }
}

impl <'a, Ctx: ReifyContext> ReifyProcessor for TeraProcessor<'a, Ctx> {
    fn render(&self, source: &str) -> anyhow::Result<String> {
        let ctx = Context::from_value(to_value(self.ctx.read_ctx())?)?;
        let result = Tera::one_off(source, &ctx, false)?;
        Ok(result)
    }
}