use anyhow::Context;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub upload_dir: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            server_host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .context("SERVER_PORT must be a valid port number")?,
            database_url: std::env::var("DATABASE_URL").context("DATABASE_URL IS REQUIRED")?,
            jwt_secret: std::env::var("JWT_SECRET").context("JWT_SECRET IS REQUIRED")?,
            upload_dir: std::env::var("UPLOAD_DIR").context("UPLOAD_DIR IS REQUIRED")?,
        })
    }
}