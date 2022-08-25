mod abac;
mod rbac;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use validator::Validate;

use crate::{
    model::cache::{Policy, Statement},
    service::bind,
    store::{cache::Cache, mysql::policy},
    Error, Result,
};

pub use abac::Abac;
pub use rbac::Rbac;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Attribute {
    // 用户
    #[validate(length(min = 1))]
    pub user_id: String,
    // 账户
    #[validate(length(min = 1))]
    pub account_id: String,
    // 用户名
    #[validate(length(min = 1))]
    pub name: String,
    // 附加信息
    pub extra: HashMap<String, Vec<String>>,
    // 动作
    #[validate(length(min = 1))]
    pub action: String,
    // 资源信息
    pub resource: Option<(String, String)>,
    // 请求路径
    #[validate(length(min = 1))]
    pub path: String,
}

impl Attribute {
    pub fn is_only_read(&self) -> bool {
        self.action.eq("get") || self.action.eq("list") || self.action.eq("watch")
    }
}

#[async_trait::async_trait]
pub trait Matchers {
    async fn authorize(&self, pool: &MySqlPool, att: &Attribute) -> Result<(Decision, String)>;
}

#[derive(Clone)]
pub enum Decision {
    Deny,
    Allow,
    Nop,
}

impl Default for Decision {
    fn default() -> Self {
        Decision::Deny
    }
}

impl ToString for Decision {
    fn to_string(&self) -> String {
        match *self {
            Decision::Deny => "Deny",
            Decision::Allow => "Allow",
            Decision::Nop => "Nop",
        }
        .to_string()
    }
}

impl std::fmt::Debug for Decision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub async fn get_policys(pool: &MySqlPool, uid: &str) -> Result<Vec<Policy>> {
    // 根据user_id查出绑定的(user_id,policy)
    let list = bind::get_id_pair_with_user(pool, uid).await?;
    let mut results = Vec::with_capacity(list.len());

    for (user_id, policy_id) in list {
        results.push(get_policy(pool, &user_id, &policy_id).await?);
    }
    Ok(results)
}

pub async fn get_policy(pool: &MySqlPool, uid: &str, policy_id: &str) -> Result<Policy> {
    match Cache.get_policy(uid, policy_id).await {
        Ok(v) => Ok(v),
        Err(err) => {
            if Error::NotFound("".to_owned()).ne(&err) {
                return Err(err);
            }
            let value = policy::get(pool, policy_id).await?;
            let result = Policy {
                id: value.id,
                version: value.version,
                statement: Statement {
                    description: value.desc,
                    subjects: value.subjects,
                    effect: value.effect,
                    action: value.action,
                    resources: value.resources,
                    collections: value.collections,
                },
            };
            Cache.set_policy(uid, &result).await?;
            Ok(result)
        }
    }
}
