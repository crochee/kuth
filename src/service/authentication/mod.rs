pub mod basic;
pub mod bearer;
pub mod sign;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Default, Deserialize)]
pub struct Claims {
    pub id: String,
    pub account_id: String,
    pub name: String,
    pub admin: u8,
    pub exp: Option<u64>,
    pub args: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct Request {
    #[validate(length(min = 1))]
    pub resource: Option<String>,
    #[validate(length(min = 1))]
    pub sub_resource: Option<String>,
    #[validate(length(min = 1))]
    pub path: Option<String>,
    #[validate(length(min = 1))]
    pub action: String,
}

#[derive(Debug, Serialize)]
pub struct Effect {
    pub user_id: String,
    pub account_id: String,
}
