use serde::{Deserialize, Serialize};

use crate::models::auth_model::Account;


#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: Account,
}
