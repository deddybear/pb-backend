use serde::{Serialize};

use crate::models::account_model::Account;


#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: Account,
}
