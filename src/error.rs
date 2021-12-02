use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug)]
pub enum ReifyError {
    #[error("JS Error - {0}")]
    JsError(String)
}

impl From<JsValue> for ReifyError {
    fn from(v: JsValue) -> Self {
        ReifyError::JsError(format!("{:?}", v))
    }
}