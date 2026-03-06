use crate::utils::errors::AppResult;
use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[garde(length(min = 4, max = 16))]
    pub username: String,
    #[garde(length(min = 5))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct SignupRequest {
    #[garde(length(min = 4, max = 16), pattern(r"^\w+$"))]
    pub username: String,
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 5))]
    pub password: String,
    #[garde(range(min = 5, max = 255))]
    pub age: i32
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AccountRecoveryRequest {
    #[garde(length(min = 4, max = 16))]
    pub username: String
}

impl AccountRecoveryRequest {
    pub fn validate(&self) -> AppResult<()> {
        Validate::validate(self).map_err(Into::into)
    }
}

impl SignupRequest {
    pub fn validate(&self) -> AppResult<()> {
        Validate::validate(self).map_err(Into::into)
    }
}

impl LoginRequest {
    pub fn validate(&self) -> AppResult<()> {
        Validate::validate(self).map_err(Into::into)
    }
}
