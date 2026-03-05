use serde::{Deserialize};

#[derive(Debug, Deserialize, Default)]
pub struct ListShopWeaponQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub search: Option<String>,
}
