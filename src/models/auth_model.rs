use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Account {
    pub player_id: i64,
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub age: i32,
    pub rank: i32,
    pub experience: i32,
    pub gold: i32,
    pub cash: i32,
    pub ribbon_count: i16,
    pub ensign_count: i16,
    pub medal_count: i16,
    pub master_medal_count: i16,
    pub create_time: DateTime<Utc>,
}
