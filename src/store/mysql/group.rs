use chrono::Utc;
use serde::Deserialize;
use sqlx::{MySql, MySqlPool, Transaction};
use validator::Validate;

use crate::{
    model::{Group, List, ID},
    utils::id::next_id,
    Error, Result,
};

#[derive(Debug, Deserialize, Validate)]
pub struct Content {
    #[serde(skip)]
    pub account_id: String,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub desc: String,
}

pub async fn create(tx: &mut Transaction<'_, MySql>, content: &Content) -> Result<ID> {
    super::account::exist(tx, &content.account_id).await?;
    let group_id = next_id().map_err(Error::any)?;
    sqlx::query!(
        r#"INSERT INTO `group`
        (`id`,`name`,`desc`,`account_id`)
        VALUES(?,?,?,?);"#,
        group_id,
        content.name,
        content.desc,
        content.account_id,
    )
    .execute(tx)
    .await
    .map_err(Error::any)?;
    Ok(ID {
        id: group_id.to_string(),
    })
}

pub async fn delete(pool: &MySqlPool, id: &str, account_id: &str) -> Result<()> {
    sqlx::query!(
        r#"UPDATE `group` SET `deleted` = `id`,`deleted_at`= ? 
        WHERE `id` = ? AND `account_id` = ? AND `deleted` = 0;"#,
        Some(Utc::now().naive_utc()),
        id,
        account_id,
    )
    .execute(pool)
    .await
    .map_err(Error::any)?;
    Ok(())
}

#[derive(Debug, Deserialize, Validate)]
pub struct Opts {
    #[validate(length(min = 1))]
    pub name: Option<String>,
    #[validate(length(min = 1))]
    pub desc: Option<String>,
}

pub async fn update(pool: MySqlPool, id: &str, account_id: &str, opts: &Opts) -> Result<()> {
    let mut update_content = String::from("");
    if let Some(name) = &opts.name {
        update_content.push_str(format!(r#"`name` = '{}'"#, name).as_str());
    };
    if let Some(desc) = &opts.desc {
        if !update_content.is_empty() {
            update_content.push_str(" , ");
        }
        update_content.push_str(format!(r#"`desc` = '{}'"#, desc).as_str());
    };
    if update_content.is_empty() {
        return Ok(());
    }
    sqlx::query(
        format!(
            r#"UPDATE `group` SET {}
            WHERE `id` = ? AND `account_id` = ? AND `deleted` = 0;"#,
            update_content
        )
        .as_str(),
    )
    .bind(id)
    .bind(account_id)
    .execute(&pool)
    .await
    .map_err(Error::any)?;
    Ok(())
}

pub async fn get(pool: MySqlPool, id: &str, account_id: &str) -> Result<Group> {
    match sqlx::query!(
        r#"SELECT `id`,`name`,`desc`,`account_id`,`created_at`,`updated_at`
        FROM `group`
        WHERE `id` = ? AND `account_id` = ? AND `deleted` = 0;"#,
        id,
        account_id,
    )
    .map(|row| Group {
        id: row.id.to_string(),
        name: row.name,
        desc: row.desc,
        account_id: row.account_id.to_string(),
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
    pub account_id: Option<String>,
    pub limit: u64,
    pub offset: u64,
    pub sort: String,
}

pub async fn list(pool: MySqlPool, opts: &ListOpts) -> Result<List<Group>> {
    match &opts.account_id {
        Some(account_id) => {
            let group_result = sqlx::query!(
                r#"SELECT COUNT(`id`) as count FROM `group`
                WHERE `account_id` = ? AND `deleted` = 0;"#,
                account_id,
            )
            .fetch_one(&pool)
            .await
            .map_err(Error::any)?;
            let groups = sqlx::query!(
                r#"SELECT `id`,`name`,`desc`,`account_id`,`created_at`,`updated_at`
                FROM `group`
                WHERE `account_id` = ? AND `deleted` = 0 ORDER BY ? LIMIT ? OFFSET ?;"#,
                account_id,
                opts.sort,
                opts.limit,
                opts.offset,
            )
            .map(|row| Group {
                id: row.id.to_string(),
                name: row.name,
                desc: row.desc,
                account_id: row.account_id.to_string(),
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .fetch_all(&pool)
            .await
            .map_err(Error::any)?;
            Ok(List {
                data: groups,
                limit: opts.limit,
                offset: opts.offset,
                total: group_result.count,
            })
        }
        None => {
            let group_result = sqlx::query!(
                r#"SELECT COUNT(`id`) as count FROM `group`
                WHERE `deleted` = 0;"#,
            )
            .fetch_one(&pool)
            .await
            .map_err(Error::any)?;
            let groups = sqlx::query!(
                r#"SELECT `id`,`name`,`desc`,`account_id`,`created_at`,`updated_at`
                FROM `group`
                WHERE `deleted` = 0 ORDER BY ? LIMIT ? OFFSET ?;"#,
                opts.sort,
                opts.limit,
                opts.offset,
            )
            .map(|row| Group {
                id: row.id.to_string(),
                name: row.name,
                desc: row.desc,
                account_id: row.account_id.to_string(),
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .fetch_all(&pool)
            .await
            .map_err(Error::any)?;
            Ok(List {
                data: groups,
                limit: opts.limit,
                offset: opts.offset,
                total: group_result.count,
            })
        }
    }
}

pub async fn exist(tx: &mut Transaction<'_, MySql>, id: &str) -> Result<()> {
    let result = sqlx::query!(
        r#"SELECT COUNT(*) as count FROM `group`
        WHERE `id` = ? AND `deleted` = 0 LIMIT 1;"#,
        id,
    )
    .fetch_one(tx)
    .await
    .map_err(Error::any)?;
    if result.count != 0 {
        return Ok(());
    }
    Err(Error::NotFound(format!("not found {}", id)))
}

pub async fn list_all_by_account_idlist(pool: &MySqlPool, account_id: &str) -> Result<Vec<Group>> {
    sqlx::query!(
        r#"SELECT `id`,`name`,`desc`,`account_id`,`created_at`,`updated_at`
        FROM `group`
        WHERE `account_id`= ? AND `deleted` = 0;"#,
        account_id,
    )
    .map(|row| Group {
        id: row.id.to_string(),
        name: row.name,
        desc: row.desc,
        account_id: row.account_id.to_string(),
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
    .fetch_all(pool)
    .await
    .map_err(Error::any)
}
