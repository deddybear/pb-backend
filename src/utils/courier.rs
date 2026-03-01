use std::error::Error;
use lettre::message::Mailbox;
use lettre::message::{MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::collections::HashMap;

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
    username_smtp: String,
    password_smtp: String,
    host_smtp: String,
    port_smtp: u16,
    name_from: String,
    email_from: String,
    name_to: String,
    email_to: String,
    subject: String,
    password_user_enc64: String,
    content_email: String,
) -> Result<(), Box<dyn Error>> {

    let mut variables: HashMap<&str, &str> = HashMap::new();

    variables.insert("nickname", &name_to);
    variables.insert("new_password", &password_user_enc64);
    variables.insert("email_user", &email_to);
    variables.insert("email_support", &email_from);
    variables.insert("year_now", "2026");

    let html_content = render_template(&content_email, &variables);

    let email = Message::builder()
        .from(Mailbox::new(Some(name_from), email_from.parse().unwrap()))
        .to(Mailbox::new(Some(name_to), email_to.parse().unwrap()))
        .subject(subject)
        .multipart(MultiPart::alternative().singlepart(SinglePart::html(html_content)))
        .unwrap();

    // create credential to smtp gmail
    let creds = Credentials::new(username_smtp, password_smtp);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&host_smtp)
        .unwrap()
        .port(port_smtp)
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e))
    }
}

/// Fungsi render template — simple string replace
fn render_template(template: &str, vars: &HashMap<&str, &str>) -> String {
    let mut result = template.to_string();
    for (key, value) in vars {
        let placeholder = format!("{{{{{}}}}}", key); // menghasilkan {{key}}
        result = result.replace(&placeholder, value);
    }
    result
}
