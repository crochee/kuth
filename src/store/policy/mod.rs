mod cache;
mod mysql;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub use cache::Cache;
pub use mysql::Mysql as PolicyStore;

#[derive(Debug, Deserialize, Serialize)]
pub struct Content {
    pub name: String,
    pub scope: String,
    pub kind: Kind,
    pub desc: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Policy {
    pub id: String,
    pub name: String,
    pub scope: String,
    pub kind: Kind,
    pub desc: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Kind {
    System,
    Custom,
}

impl From<String> for Kind {
    fn from(value: String) -> Self {
        if value.eq("System") {
            return Self::System;
        }
        if value.eq("Custom") {
            return Self::Custom;
        }
        Self::Custom
    }
}

impl ToString for Kind {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug)]
pub struct Query {
    pub kind: Option<Kind>,
    pub limit: u64,
    pub offset: u64,
    pub sort: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::store::Store;

    use super::*;

    #[tokio::test]
    async fn create() {
        let object = PolicyStore;
        let id = object
            .create(&Content {
                name: "test".to_owned(),
                scope: "global".to_owned(),
                kind: Kind::System,
                desc: "any way desc".to_string(),
                content: r#"{"key":"value"}"#.to_owned(),
            })
            .await
            .unwrap();

        let c = object.get(&id.id).await.unwrap();
        println!("{:#?}", c)
    }
}
