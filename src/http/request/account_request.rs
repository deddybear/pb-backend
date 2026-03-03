use crate::utils::errors::AppResult;
use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ChangePassword {
    #[garde(range(min = 1))]
    pub player_id: i32,
    #[garde(length(min = 5))]
    pub old_password: String,
    #[garde(length(min = 5))]
    pub new_password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ChangeEmail {
    #[garde(range(min = 1))]
    pub player_id: i32,
    #[garde(pattern(r"[a-z]+"))]
    pub new_email: String,
}


impl ChangePassword {
    pub fn validate(&self) -> AppResult<()> {
        Validate::validate(self).map_err(Into::into)
    }
}

impl ChangeEmail {
    pub fn validate(&self) -> AppResult<()> {
        Validate::validate(self).map_err(Into::into)
    }
}