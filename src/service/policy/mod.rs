use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use validator::Validate;

use crate::{
    model::List,
    service,
    store::{cache::Cache, mysql::policy},
    Result,
};

pub async fn delete(pool: MySqlPool, id: &str) -> Result<()> {
    policy::delete(&pool, id).await?;
    let pairs = service::bind::get_id_pair_with_policy(&pool, id).await?;
    for (user_id, policy_id) in pairs {
        Cache.delete_policy(&user_id, &policy_id).await?;
    }
    Ok(())
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Policy {
    pub name: String,
    pub scope: String,
    pub kind: Kind,
    pub desc: String,
    pub content: Content,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Kind {
    System,
    Custom,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Content {
    // 指定要使用的策略语言版本
    pub version: String,
    pub statement: Vec<Statement>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Statement {
    pub sid: Option<String>,
    pub effect: Effect,
    pub principal: Option<HashMap<String, Vec<String>>>,
    pub action: Vec<String>,
    pub resource: Vec<String>,
    pub condition: Option<HashMap<String, HashMap<String, Vec<String>>>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Effect {
    Allow,
    Deny,
}

#[async_trait::async_trait]
pub trait PolicySrv {
    async fn create(&self, content: &Policy) -> Result<String>;
    async fn get(&self, id: &str) -> Result<()>;
    async fn delete(&self, id: &str) -> Result<()>;
    async fn list(&self, id: &str) -> Result<List<Policy>>;
}
