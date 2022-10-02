use chrono::Utc;
use rand::Rng;
use serde::Deserialize;
use sqlx::{MySql, MySqlPool, Transaction};
use validator::Validate;

use crate::{
    model::{List, Secret, ID},
    utils::id::next_id,
    Error, Result,
};

#[derive(Debug, Deserialize, Validate)]
pub struct Content {
    #[serde(skip)]
    pub user_id: String,
    #[validate(length(min = 1))]
    pub name: String,
    pub expire: Option<u64>,
}

pub async fn create(tx: &mut Transaction<'_, MySql>, content: &Content) -> Result<ID> {
    super::user::exist(tx, &content.user_id).await?;
    let secret_id = next_id().map_err(Error::any)?;
    let ak = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(24)
        .map(char::from)
        .collect::<String>();
    let sk = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(64)
        .map(char::from)
        .collect::<String>();
    let mut expire = content.expire.unwrap_or_default() as i64;
    if expire > 0 {
        expire += Utc::now().timestamp();
    }
    sqlx::query!(
        r#"INSERT INTO `secret`
        (`id`,`name`,`user_id`,`access_key`,`secret_access_key`,`expire`)
        VALUES(?,?,?,?,?,?);"#,
        secret_id,
        content.name,
        content.user_id,
        ak,
        sk,
        expire,
    )
    .execute(tx)
    .await
    .map_err(Error::any)?;
    Ok(ID {
        id: secret_id.to_string(),
    })
}

pub async fn delete(pool: &MySqlPool, id: &str, user_id: &str) -> Result<()> {
    sqlx::query!(
        r#"UPDATE `secret` SET `deleted` = `id`,`deleted_at`= ? 
        WHERE `id` = ? AND `user_id` = ? AND `deleted` = 0;"#,
        Some(Utc::now().naive_utc()),
        id,
        user_id,
    )
    .execute(pool)
    .await
    .map_err(Error::any)?;
    Ok(())
}

pub async fn update(pool: MySqlPool, id: &str, user_id: &str, name: &Option<String>) -> Result<()> {
    if let Some(v) = name {
        sqlx::query!(
            r#"UPDATE `secret` SET `name`= ?
            WHERE `id` = ? AND `user_id` = ? AND `deleted` = 0;"#,
            v,
            id,
            user_id,
        )
        .execute(&pool)
        .await
        .map_err(Error::any)?;
    }
    Ok(())
}

pub async fn get(pool: MySqlPool, id: &str, user_id: &str) -> Result<Secret> {
    match sqlx::query!(
        r#"SELECT `id`,`name`,`user_id`,`access_key`,`expire`,`created_at`,`updated_at`
        FROM `secret`
        WHERE `id` = ? AND `user_id` = ? AND `deleted` = 0;"#,
        id,
        user_id,
    )
    .map(|row| Secret {
        id: row.id.to_string(),
        name: row.name,
        user_id: row.user_id.to_string(),
        access_key: row.access_key,
        expire: row.expire,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
    .fetch_optional(&pool)
    .await
    {
        Ok(v) => match v {
            Some(value) => Ok(value),
            None => Err(Error::NotFound("no rows".to_owned())),
        },
        Err(err) => Err(Error::any(err)),
    }
}

pub struct ListOpts {
    pub user_id: Option<String>,
    pub limit: u64,
    pub offset: u64,
    pub sort: String,
}

pub async fn list(pool: &MySqlPool, opts: &ListOpts) -> Result<List<Secret>> {
    match &opts.user_id {
        Some(user_id) => {
            let secret_result = sqlx::query!(
                r#"SELECT COUNT(`id`) as count FROM `secret`
                WHERE `user_id` = ? AND `deleted` = 0;"#,
                user_id,
            )
            .fetch_one(pool)
            .await
            .map_err(Error::any)?;
            let users = sqlx::query!(
                r#"SELECT `id`,`name`,`user_id`,`access_key`,`expire`,`created_at`,`updated_at`
                FROM `secret`
                WHERE `user_id` = ? AND `deleted` = 0 ORDER BY ? LIMIT ? OFFSET ?;"#,
                user_id,
                opts.sort,
                opts.limit,
                opts.offset,
            )
            .map(|row| Secret {
                id: row.id.to_string(),
                name: row.name,
                user_id: row.user_id.to_string(),
                access_key: row.access_key,
                expire: row.expire,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .fetch_all(pool)
            .await
            .map_err(Error::any)?;
            Ok(List {
                data: users,
                limit: opts.limit,
                offset: opts.offset,
                total: secret_result.count,
            })
        }
        None => {
            let secret_result = sqlx::query!(
                r#"SELECT COUNT(`id`) as count FROM `secret`
                WHERE `deleted` = 0;"#,
            )
            .fetch_one(pool)
            .await
            .map_err(Error::any)?;
            let users = sqlx::query!(
                r#"SELECT `id`,`name`,`user_id`,`access_key`,`expire`,`created_at`,`updated_at`
                FROM `secret`
                WHERE `deleted` = 0 ORDER BY ? LIMIT ? OFFSET ?;"#,
                opts.sort,
                opts.limit,
                opts.offset,
            )
            .map(|row| Secret {
                id: row.id.to_string(),
                name: row.name,
                user_id: row.user_id.to_string(),
                access_key: row.access_key,
                expire: row.expire,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .fetch_all(pool)
            .await
            .map_err(Error::any)?;
            Ok(List {
                data: users,
                limit: opts.limit,
                offset: opts.offset,
                total: secret_result.count,
            })
        }
    }
}

pub async fn list_all_by_user_id(pool: &MySqlPool, user_id: &str) -> Result<Vec<Secret>> {
    sqlx::query!(
        r#"SELECT `id`,`name`,`user_id`,`access_key`,`expire`,`created_at`,`updated_at`
        FROM `secret`
        WHERE `user_id` = ? AND `deleted` = 0;"#,
        user_id,
    )
    .map(|row| Secret {
        id: row.id.to_string(),
        name: row.name,
        user_id: row.user_id.to_string(),
        access_key: row.access_key,
        expire: row.expire,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
    .fetch_all(pool)
    .await
    .map_err(Error::any)
}
