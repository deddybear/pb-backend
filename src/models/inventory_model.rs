use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct StateAccountMoney {
    pub cash: i32,
    pub gold: i32,
    pub tag: i32
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct StateAccountMedal {
    pub cash: i32,
    pub gold: i32,
    pub tag: i32
}
