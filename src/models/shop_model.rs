use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ShopWeapon {
    pub item_id: i32,
    pub item_name: String,
    pub item_consume: i32,
    pub item_count_list: String,
    pub price_cash_list: String,
    pub price_gold_list: String,
    pub item_visible: bool,
    pub discount_percent: i32
}
