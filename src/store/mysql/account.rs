use chrono::Utc;
use serde::Deserialize;
use sqlx::{MySql, MySqlPool, Transaction};
use validator::Validate;

use crate::{
    model::{Account, ID},
    utils::id::next_id,
    Error, Result,
};

#[derive(Debug, Deserialize, Validate)]
pub struct Content {
    #[validate(length(min = 1))]
    pub account_name: String,
    #[validate(length(min = 1))]
    pub account_desc: String,
}

pub async fn create(tx: &mut Transaction<'_, MySql>, content: &Content) -> Result<ID> {
    let account_id = next_id().map_err(Error::any)?;
    sqlx::query!(
        r#"INSERT INTO `account`
        (`id`,`name`,`desc`)
        VALUES(?,?,?);"#,
        account_id,
        content.account_name,
        content.account_desc,
    )
    .execute(tx)
    .await
    .map_err(Error::any)?;
    Ok(ID {
        id: account_id.to_string(),
    })
}

pub async fn delete(pool: &MySqlPool, id: &str) -> Result<()> {
    sqlx::query!(
        r#"UPDATE `account` SET `deleted` = `id`,`deleted_at`= ? 
        WHERE `id` = ? AND `deleted` = 0;"#,
        Some(Utc::now().naive_utc()),
        id,
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

pub async fn update(pool: MySqlPool, id: &str, opts: &Opts) -> Result<()> {
    let mut update_content = String::from("");
    if let Some(name) = &opts.name {
        update_content.push_str(format!(r#"`name` = '{}'"#, name).as_str());
    }
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
            r#"UPDATE `account` SET {}
                WHERE `id` = ? AND `deleted` = 0;"#,
            update_content
        )
        .as_str(),
    )
    .bind(id)
    .execute(&pool)
    .await
    .map_err(Error::any)?;
    Ok(())
}

pub async fn get(pool: MySqlPool, id: &str) -> Result<Account> {
    match sqlx::query!(
        r#"SELECT `id`,`name`,`desc`,`created_at`,`updated_at` FROM `account`
        WHERE `id` = ? AND `deleted` = 0;"#,
        id,
    )
    .map(|row| Account {
        id: row.id.to_string(),
        name: row.name,
        desc: row.desc,
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

pub async fn exist(tx: &mut Transaction<'_, MySql>, id: &str) -> Result<()> {
    let result = sqlx::query!(
        r#"SELECT COUNT(*) as count FROM `account`
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
