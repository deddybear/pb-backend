use crate::utils::errors::AppResult;
use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct TopUpMoneyRequest {
    #[garde(range(min = 1))]
    pub player_id: i32,
    #[garde(pattern(r"[a-z]+"))]
    pub top_up_type: String,
    #[garde(range(min = 1))]
    pub value: i32,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct TopUpMedalRequest {
    #[garde(range(min = 1))]
    pub player_id: i32,
    #[garde(pattern(r"[a-z]+"))]
    pub top_up_type: String,
    #[garde(range(min = 1))]
    pub value: i32,
}

// #[derive(Debug, Deserialize, Serialize, Validate)]
// pub struct BuyWeaponRequest {
//     #[garde(range(min = 1))]
//     pub player_id: i64,
//     #[garde(range(min = 1))]
//     pub weapon_id: i32,
//     #[garde(pattern(r"[a-z]+"))]
//     pub name_weapon: String,
//     #[garde(range(min = 1))]
//     pub count: i64,
//     #[garde(range(min = 1))]
//     pub equip_slot: i32,
// }

impl TopUpMoneyRequest {
    pub fn validate(&self) -> AppResult<()> {
        Validate::validate(self).map_err(Into::into)
    }
}

impl TopUpMedalRequest {
    pub fn validate(&self) -> AppResult<()> {
        Validate::validate(self).map_err(Into::into)
    }
}

// impl BuyWeaponRequest {
//     pub fn validate(&self) -> AppResult<()> {
//         Validate::validate(self).map_err(Into::into)
//     }
// }
