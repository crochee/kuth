use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use validator::Validate;

use crate::{
    store::mysql::{account, group, user},
    Result,
};

#[derive(Debug, Deserialize, Validate)]
pub struct Content {
    #[serde(flatten)]
    #[validate]
    pub account: account::Content,
    #[serde(flatten)]
    #[validate]
    pub user: user::Content,
}

#[derive(Debug, Serialize)]
pub struct Info {
    pub account_id: String,
    pub user_id: String,
}

pub async fn create(pool: MySqlPool, content: &mut Content) -> Result<Info> {
    let account_id = account::create(&pool, &content.account).await?;

    content.user.account_id = account_id.id.clone();
    content.user.admin = 2;
    let user_id = user::create(&pool, &content.user).await?;
    Ok(Info {
        account_id: account_id.id,
        user_id: user_id.id,
    })
}

pub async fn delete(pool: MySqlPool, id: &str) -> Result<()> {
    // 删除account
    account::delete(&pool, id).await?;
    // 删除user
    let users = user::list_all_by_account_id(&pool, id).await?;
    for value in users {
        super::user::delete(&pool, &value.id, id).await?;
    }
    // 删除group
    let groups = group::list_all_by_account_idlist(&pool, id).await?;
    for value in groups {
        super::group::delete(&pool, &value.id, id).await?;
    }
    Ok(())
}
