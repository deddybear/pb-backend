use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Template {
    pub code_http: u16,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TemplateWithData {
    pub code_http: u16,
    pub message: String,
    pub response: Option<Value>,
}