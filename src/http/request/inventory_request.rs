use crate::utils::errors::AppResult;
use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct GetListRequest {
    #[garde(range(min = 1))]
    pub player_id: i32,
    #[garde(pattern(r"[a-z]+"))]
    pub top_up_type: String,
    #[garde(range(min = 1))]
    pub value: i32
}

impl GetListRequest {
    pub fn validate(&self) -> AppResult<()> {
        Validate::validate(self).map_err(Into::into)
    }
}
