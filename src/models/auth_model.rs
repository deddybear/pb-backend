use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

use crate::utils::errors::AppResult;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 2, max = 100, message = "Name must be 2-100 characters"))]
    pub username: String,
    #[validate(length(min = 5, message = "Password must be at least 5 characters"))]
    pub password: String,
}

impl LoginRequest {
    pub fn validate(&self) -> AppResult<()> {
        Validate::validate(self).map_err(Into::into)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignupRequest {
    pub username: String,
    pub email: String,
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

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Account {
    pub player_id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub age: i32,
    pub rank: i8,
    pub ip4_address: String,
    pub mac_address: String,
    pub gold: i64,
    pub cash: i64,
    pub ribbon_count: i16,
    pub ensign_count: i16,
    pub medal_count: i16,
    pub master_medal_count: i16,
    pub create_time: DateTime<Utc>,
}
