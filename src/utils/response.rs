use crate::http::response::general_response::{Template, TemplateWithData};

/// Creates a basic HTTP response template with status code and message.
///
/// # Arguments
///
/// * `code` - HTTP status code (e.g., 200, 404, 500)
/// * `message` - Response message string
///
/// # Returns
///
/// A `Template` struct containing the HTTP code and message

pub fn create_response(code: u16, message: &str) -> Template {
    Template {
        code_http: code,
        message: message.to_string(),
    }
}

/// Creates an HTTP response template with status code, message, and optional data payload.
///
/// # Arguments
///
/// * `code` - HTTP status code (e.g., 200, 404, 500)
/// * `message` - Response message string
/// * `data` - Optional JSON value containing response data
///
/// # Returns
///
/// A `TemplateWithData` struct containing the HTTP code, message, and response data
pub fn create_response_with_data(
    code: u16,
    message: &str,
    data: Option<serde_json::Value>,
) -> TemplateWithData {
    TemplateWithData {
        code_http: code,
        message: message.to_string(),
        response: data,
    }
}
