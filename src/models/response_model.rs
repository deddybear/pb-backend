use::serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct Template {
    pub code_http: u16,
    pub message: String,
}

#[derive(Serialize)]
pub struct TemplateWithData {
    pub code_http: u16,
    pub message: String,
    pub data: Option<Value>,
}