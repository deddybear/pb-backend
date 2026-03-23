use crate::utils::errors::AppResult;
use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct GetDataAccountQuery {
    #[garde(range(min = 1))]
    pub player_id: i32,
}

impl GetDataAccountQuery {
    pub fn validate(&self) -> AppResult<()> {
        Validate::validate(self).map_err(Into::into)
    }
}
