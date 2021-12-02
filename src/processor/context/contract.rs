use std::collections::HashMap;

pub trait ReifyContext {
    fn read_ctx(&self) -> &HashMap<String, String>;
}