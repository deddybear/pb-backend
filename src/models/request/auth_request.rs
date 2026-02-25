use crate::utils::errors::AppResult;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 2, max = 100, message = "Name must be 2-100 characters"))]
    pub username: String,
    #[validate(length(min = 5, message = "Password must be at least 5 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct SignupRequest {
    #[validate(length(min = 2, max = 100, message = "Name must be 2-100 characters"))]
    pub username: String,
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
    #[validate(length(min = 5, message = "Password must be at least 5 characters"))]
    pub password: String,
    pub age: u8,
    pub rank: u8,
    pub ip4_address: String,
    pub mac_address: String,
    pub gold: u64,
    pub cash: u64,
    pub ribbon_count: u16,
    pub ensign_count: u16,
    pub medal_count: u16,
    pub master_medal_count: u16,
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





