use std::collections::HashMap;

pub trait ReifyContext {
    fn read_ctx(&self) -> &HashMap<String, String>;
}

pub struct EnvContext {
    context: HashMap<String, String>,
}

impl EnvContext {
    pub fn new(prefix: &str) -> anyhow::Result<Self> {
        Ok(Self {
            context: envy::prefixed(format_prefix(prefix)).from_env()?,
        })
    }

    pub fn merge_default(defaults: HashMap<String, String>, prefix: &str) -> anyhow::Result<Self> {
        let fmt_prefix = format_prefix(prefix);
        let mut ctx = Self::new(&fmt_prefix)?;
        defaults.into_iter()
            .filter_map(|(k, v)|
                if k.starts_with(&fmt_prefix) {
                    Some((k[fmt_prefix.len()..].to_string(), v))
                } else {
                    None
                })
            .for_each(|(k, v)| { ctx.context.entry(k).or_insert(v); });

        Ok(ctx)
    }
}

fn format_prefix(prefix: &str) -> String {
    if prefix.ends_with('_') {
        prefix.to_string()
    } else {
        format!("{}_", prefix)
    }
}

impl ReifyContext for EnvContext {
    fn read_ctx(&self) -> &HashMap<String, String> {
        &self.context
    }
}