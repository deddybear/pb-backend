use crate::AppState;
use crate::utils::errors::AppError;
use lettre::message::Mailbox;
use lettre::message::{MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::collections::HashMap;
use std::error::Error;

/// Sends an email with HTML content via SMTP.
///
/// This function constructs an email message and sends it through Gmail's SMTP server
/// using the STARTTLS protocol.
///
/// # Arguments
///
/// * `name_from` - The display name of the sender
/// * `email_from` - The email address of the sender
/// * `name_to` - The display name of the recipient
/// * `email_to` - The email address of the recipient
/// * `subject` - The subject line of the email
/// * `template_email` - The HTML content of the email body
///
/// # Returns
///
/// A `String` containing either a success message ("Email sent successfully!")
/// or an error message describing the failure.
///
/// # Panics
///
/// This function will panic if:
/// - The sender's email address fails to parse
/// - The recipient's email address fails to parse
/// - The SMTP connection cannot be established
///
/// # Example
///
/// ```ignore
/// send_mail(
///     "John Doe".to_string(),
///     "john@example.com".to_string(),
///     "Jane Smith".to_string(),
///     "jane@example.com".to_string(),
///     "Hello Jane".to_string(),
///     "<h1>Hi Jane!</h1>".to_string(),
/// );
/// ```
pub fn send_mail(
    state: AppState,
    name_from: String,
    name_to: String,
    email_to: String,
    subject: String,
    content_email: String,
) -> Result<(), Box<dyn Error>> {
    // init email
    let email = Message::builder()
        .from(Mailbox::new(Some(name_from),state.config.smtp_address_from.parse()?))
        .to(Mailbox::new(Some(name_to), email_to.parse()?))
        .subject(subject)
        .multipart(MultiPart::alternative().singlepart(SinglePart::html(content_email)))
        .map_err(|e| AppError::InternalError(format!("Email build error: {}", e.to_string())))?;

    // create credential to smtp gmail
    let creds = Credentials::new(
        state.config.smtp_username.clone(),
        state.config.smtp_password.clone(),
    );

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&state.config.smtp_host)
        .unwrap()
        .port(state.config.smtp_port)
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

/// Fungsi render template — simple string replace
pub fn render_template(template: &str, vars: &HashMap<&str, &str>) -> String {
    let mut result = template.to_string();
    for (key, value) in vars {
        let placeholder = format!("{{{{{}}}}}", key); // menghasilkan {{key}}
        result = result.replace(&placeholder, value);
    }
    result
}
