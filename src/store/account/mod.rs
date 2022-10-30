mod mysql;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub use mysql::Mysql as AccountStore;

#[derive(Debug, Deserialize, Serialize)]
pub struct Content {
    pub name: String,
    pub desc: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct Opts {
    #[validate(length(min = 1))]
    pub name: Option<String>,
    #[validate(length(min = 1))]
    pub desc: Option<String>,
}

#[derive(Debug)]
pub struct Query {
    pub limit: u64,
    pub offset: u64,
    pub sort: Option<String>,
}
