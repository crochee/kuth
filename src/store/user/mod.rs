mod mysql;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub use mysql::Mysql as UserStore;

use crate::model::param::check_password;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Content {
    #[serde(skip)]
    pub account_id: String,
    #[serde(skip, default)]
    pub admin: bool,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(custom = "check_password")]
    pub password: String,
    #[validate(length(min = 1))]
    pub desc: String,
    #[validate(email)]
    pub email: Option<String>,
    pub sex: Option<Sex>,
    #[validate(url)]
    pub image: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Sex {
    Male,
    Female,
}

impl From<String> for Sex {
    fn from(value: String) -> Self {
        if value.eq("Male") {
            return Self::Male;
        }
        if value.eq("Female") {
            return Self::Female;
        }
        Self::Male
    }
}

#[derive(Debug)]
pub struct IDs {
    pub id: String,
    pub account_id: String,
}

impl ToString for IDs {
    fn to_string(&self) -> String {
        self.id.clone()
    }
}

impl ToString for Sex {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub account_id: String,
    pub admin: bool,
    pub name: String,
    pub desc: String,
    pub email: Option<String>,
    pub check: bool,
    pub sex: Option<String>,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct Opts {
    #[validate(length(min = 1))]
    pub name: Option<String>,
    #[validate(custom = "check_password")]
    pub password: Option<String>,
    #[validate(length(min = 1))]
    pub desc: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub check: Option<bool>,
    pub sex: Option<Sex>,
    #[validate(url)]
    pub image: Option<String>,
}

#[derive(Debug)]
pub struct Query {
    pub account_id: Option<String>,
    pub limit: u64,
    pub offset: u64,
    pub sort: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sex() {
        assert_eq!(Sex::Male.to_string(), "Male".to_string());
        let value: Sex = "Male".to_string().into();
        assert_eq!(value, Sex::Male);
    }
}
