use anyhow::Context;
use std::env::var;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
    pub smtp_enable: bool,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_address_from: String,
    pub jwt_secret: String,
    pub upload_dir: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            server_host: var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: var("APP_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .context("SERVER_PORT must be a valid port number")?,
            database_url: var("DATABASE_URL").context("DATABASE_URL IS REQUIRED")?,
            smtp_enable: var("MAILER_SMTP_ENABLE")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .context("SMTP_ENBALE must be a valid boolean")?,
            smtp_host : var("MAILER_SMTP_HOST").context("MAILER_SMTP_HOST IS REQUIRED")?,
            smtp_port : var("MAILER_SMTP_PORT").unwrap_or_else(|_| "465".to_string()).parse().context("MAILER_SMTP_HOST IS REQUIRED")?,
            smtp_username: var("MAILER_SMTP_USER").context("MAILER_SMTP_USER IS REQUIRED")?,
            smtp_password: var("MAILER_SMTP_PASSWORD").context("MAILER_SMTP_PASSWORD IS REQUIRED")?,
            smtp_address_from: var("MAILER_ADDRESS_FROM").context("MAILER_ADDRESS_FROM IS REQUIRED")?,
            jwt_secret: var("JWT_SECRET").context("JWT_SECRET IS REQUIRED")?,
            upload_dir: var("SERVER_UPLOAD_DIR")
                .context("SERVER_UPLOAD_DIR IS REQUIRED")?,
        })
    }
}
