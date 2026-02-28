use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // user id
    pub email: String,
    pub access_level: String,
    pub exp: i64,
    pub iat: i64,
}

/// JWT token claims structure containing user information and token metadata.
///
/// # Fields
///
/// * `sub` - Subject claim containing the user ID as a string
/// * `email` - User's email address
/// * `access_level` - User's access level permission
/// * `exp` - Token expiration time as Unix timestamp
/// * `iat` - Token issued-at time as Unix timestamp
pub fn generate_token(
    user_id: &i64,
    email: &str,
    access_level: &i32,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let exp = now + Duration::days(7);

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        access_level: access_level.to_string(),
        exp: exp.timestamp(),
        iat: now.timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}
