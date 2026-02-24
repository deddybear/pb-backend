use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
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