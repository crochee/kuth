use chrono::NaiveDateTime;
use serde::Serialize;

pub mod cache;
pub mod param;

#[derive(Debug, Serialize)]
pub struct List<T> {
    pub data: Vec<T>,
    pub limit: u64,
    pub offset: u64,
    pub total: i64,
}

#[derive(Debug, Serialize)]
pub struct ID {
    pub id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: String,
    pub account_id: String,
    pub admin: u8,
    pub name: String,
    pub desc: String,
    pub email: Option<String>,
    pub check: u8,
    pub sex: Option<String>,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserAll {
    pub id: String,
    pub account_id: String,
    pub admin: u8,
    pub name: String,
    pub desc: String,
    pub email: Option<String>,
    pub check: u8,
    pub sex: Option<String>,
    pub password: String,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize)]
pub struct Secret {
    pub id: String,
    pub name: String,
    pub user_id: String,
    pub access_key: String,
    pub expire: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub account_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize)]
pub struct Policy {
    pub id: String,
    pub desc: String,
    pub version: String,
    pub policy_type: u8,
    pub subjects: Vec<String>,
    pub effect: String,
    pub action: Vec<String>,
    pub resources: Vec<String>,
    pub collections: Vec<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize)]
pub struct Bind {
    pub id: String,
    pub group_id: String,
    pub bind_type: u8,
    pub object_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
