use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use validator::Validate;

use crate::{
    store::mysql::{account, bind, group, policy, secret, user},
    Error, Result,
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
    let mut tx = pool.begin().await.map_err(Error::any)?;
    // 创建账户
    let account_id = account::create(&mut tx, &content.account).await?;
    content.user.account_id = account_id.id.clone();
    content.user.admin = 2;
    // 创建角色
    let user_id = user::create(&mut tx, &content.user).await?;
    // 创建密钥
    secret::create(
        &mut tx,
        &secret::Content {
            user_id: user_id.id.clone(),
            name: "Administrator".to_owned(),
            expire: None,
        },
    )
    .await?;
    // 创建用户组
    let group_id = group::create(
        &mut tx,
        &group::Content {
            account_id: account_id.id.clone(),
            name: "Administrator".to_owned(),
            desc: "Administrator description".to_owned(),
        },
    )
    .await?;
    // 创建策略
    let policy_id = policy::insert_or_update(
        &mut tx,
        &policy::Content {
            name: "Admin".to_owned(),
            desc: "Admin policy".to_owned(),
            version: "v1.0.0".to_owned(),
            policy_type: 2,
            subjects: vec!["user:*".to_owned(), "account:*".to_owned()],
            effect: "Allow".to_owned(),
            action: vec!["*".to_owned()],
            resources: vec!["*".to_owned()],
            collections: vec![],
        },
    )
    .await?;
    // 创建绑定关系
    bind::create(
        &mut tx,
        &bind::Content {
            group_id: group_id.id.clone(),
            bind_type: 1,
            object_id: user_id.id.clone(),
        },
    )
    .await?;
    bind::create(
        &mut tx,
        &bind::Content {
            group_id: group_id.id,
            bind_type: 2,
            object_id: policy_id.id,
        },
    )
    .await?;
    tx.commit().await.map_err(Error::any)?;
    Ok(Info {
        account_id: account_id.id,
        user_id: user_id.id,
    })
}

pub async fn delete(pool: MySqlPool, id: &str) -> Result<()> {
    for value in user::list_all_by_account_id(&pool, id).await? {
        // 删除密钥
        for secret_value in secret::list_all_by_user_id(&pool, &value.id).await? {
            secret::delete(&pool, &secret_value.id, &value.id).await?;
        }
        // 删除user
        super::user::delete(&pool, &value.id, id).await?;
    }

    for value in group::list_all_by_account_idlist(&pool, id).await? {
        for bind_value in bind::list_all(
            &pool,
            &bind::QueryOpts {
                group_id: vec![value.id.clone()],
                bind_type: None,
                object_id: None,
            },
        )
        .await?
        {
            // 删除绑定关系
            bind::delete(&pool, &bind_value.id).await?;
        }
        // 删除group
        super::group::delete(&pool, &value.id, id).await?;
    }
    // 删除account
    account::delete(&pool, id).await
}
