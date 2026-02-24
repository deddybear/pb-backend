use crate::models::response_model::{Template, TemplateWithData};

pub fn create_response(code: u16, message: &str) -> Template {
    Template {
        code_http: code,
        message: message.to_string(),
    }
}

pub fn create_response_with_data(
    code: u16,
    message: &str,
    data: Option<serde_json::Value>,
) -> TemplateWithData {
    TemplateWithData {
        code_http: code,
        message: message.to_string(),
        data,
    }
}
