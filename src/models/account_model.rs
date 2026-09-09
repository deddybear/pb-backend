use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct AccountChangePassword {
    pub password: String,
    pub email: String,
    pub nickname: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct AccountChangeEmail {
    pub email: String,
    pub nickname: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Account {
    pub player_id: i64,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    // #[serde(skip_serializing)]
    // pub password_text: Option<String>,
    pub email: String,
    pub age: i32,
    pub rank: i32,
    pub gold: i32,
    pub cash: i32,
    pub experience: i32,
    pub nickname: String,
    pub pc_cafe: i32,
    pub access_level: i32,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>,
    #[sqlx(skip)]
    pub token: Option<String>, // pub ribbon_count: i16,
                               // pub ensign_count: i16,
                               // pub medal_count: i16,
                               // pub master_medal_count: i16,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct DataLogin {
    pub player_id: i64,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_serializing)]
    pub password_text: Option<String>,
    pub token: Option<String>
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct MedalAccount {
    pub update_time: Option<NaiveDateTime>,
    pub ribbon: i32,
    pub ensign: i32,
    pub medal: i32,
    pub master_medal: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct AccountPasswordReset {
    pub player_id: i64,
    pub email: String,
    pub nickname: String,
}
