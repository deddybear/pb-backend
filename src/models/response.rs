use::serde::Serialize;


#[derive(Serialize)]
pub struct Template {
    pub code_http: u8,
    pub message: String,
}

#[derive(Serialize)]
pub struct TemplateWithData {
    pub code_http: u8,
    pub message: String,
    pub data: Option<serde_json::Value>,
}