use crate::models::response::{Template, TemplateWithData};

pub fn create_response(code: u8, message: &str) -> Template {
    Template {
        code_http: code,
        message: message.to_string(),
    }
}

pub fn create_response_with_data(
    code: u8,
    message: &str,
    data: Option<serde_json::Value>,
) -> TemplateWithData {
    TemplateWithData {
        code_http: code,
        message: message.to_string(),
        data,
    }
}
