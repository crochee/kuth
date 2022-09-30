use chrono::Utc;
use serde::Deserialize;
use sqlx::{MySql, MySqlPool, Row, Transaction};
use validator::Validate;

use crate::{
    model::{Bind, List, ID},
    utils::id::next_id,
    Error, Result,
};

#[derive(Debug, Deserialize, Validate)]
pub struct Content {
    #[validate(length(min = 1))]
    pub group_id: String,
    #[validate(range(min = 1, max = 2))]
    pub bind_type: u8,
    #[validate(length(min = 1))]
    pub object_id: String,
}

pub async fn create(tx: &mut Transaction<'_, MySql>, content: &Content) -> Result<ID> {
    super::group::exist(tx, &content.group_id).await?;
    match content.bind_type {
        1 => {
            super::user::exist(tx, &content.object_id).await?;
        }
        2 => {
            super::policy::exist(tx, &content.object_id).await?;
        }
        _ => {
            return Err(Error::BadRequest(format!(
                "bind_type is {}",
                content.bind_type
            )))
        }
    };
    let bind_id = next_id().map_err(Error::any)?;
    sqlx::query!(
        r#"INSERT INTO `bind`
        (`id`,`group_id`,`bind_type`,`object_id`)
        VALUES(?,?,?,?);"#,
        bind_id,
        content.group_id,
        content.bind_type,
        content.object_id,
    )
    .execute(tx)
    .await
    .map_err(Error::any)?;
    Ok(ID {
        id: bind_id.to_string(),
    })
}

pub async fn delete(pool: &MySqlPool, id: &str) -> Result<()> {
    sqlx::query!(
        r#"UPDATE `bind` SET `deleted` = `id`,`deleted_at`= ? 
        WHERE `id` = ? AND `deleted` = 0;"#,
        Some(Utc::now().naive_utc()),
        id,
    )
    .execute(pool)
    .await
    .map_err(Error::any)?;
    Ok(())
}

pub async fn get(pool: &MySqlPool, id: &str) -> Result<Bind> {
    match sqlx::query!(
        r#"SELECT `id`,`group_id`,`bind_type`,`object_id`,`created_at`,`updated_at`
        FROM `bind`
        WHERE `id` = ? AND `deleted` = 0;"#,
        id,
    )
    .map(|raw| Bind {
        id: raw.id.to_string(),
        group_id: raw.group_id.to_string(),
        bind_type: raw.bind_type,
        object_id: raw.object_id.to_string(),
        created_at: raw.created_at,
        updated_at: raw.updated_at,
    })
    .fetch_optional(pool)
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
    pub group_id: Option<String>,
    pub bind_type: Option<u8>,
    pub object_id: Option<String>,
    pub limit: u64,
    pub offset: u64,
    pub sort: String,
}

pub async fn list(pool: MySqlPool, opts: &ListOpts) -> Result<List<Bind>> {
    let mut wheres = String::from("");
    if let Some(group_id) = &opts.group_id {
        wheres.push_str(format!(r#"`group_id` = {}"#, group_id).as_str());
    }
    if let Some(bind_type) = opts.bind_type {
        if !wheres.is_empty() {
            wheres.push_str(" AND ");
        }
        wheres.push_str(format!(r#"`bind_type` = {}"#, bind_type).as_str());
    };
    if let Some(object_id) = &opts.object_id {
        if !wheres.is_empty() {
            wheres.push_str(" AND ");
        }
        wheres.push_str(format!(r#"`object_id` = {}"#, object_id).as_str());
    };
    if !wheres.is_empty() {
        wheres.push_str(" AND ");
    }
    wheres.push_str(r#"`deleted` = 0"#);
    let bind_result = sqlx::query(
        format!(
            r#"SELECT COUNT(*) as count FROM `group`
            WHERE {};"#,
            wheres,
        )
        .as_str(),
    )
    .fetch_one(&pool)
    .await
    .map_err(Error::any)?;
    let rows = sqlx::query(
        format!(
            r#"SELECT `id`,`group_id`,`bind_type`,`object_id`,`created_at`,`updated_at`
            FROM `bind`
            WHERE {} ORDER BY ? LIMIT ? OFFSET ?;"#,
            wheres,
        )
        .as_str(),
    )
    .bind(opts.sort.clone())
    .bind(opts.limit)
    .bind(opts.offset)
    .fetch_all(&pool)
    .await
    .map_err(Error::any)?;
    let mut result = List {
        data: Vec::new(),
        limit: opts.limit,
        offset: opts.offset,
        total: bind_result.try_get("count").map_err(Error::any)?,
    };
    for row in rows.iter() {
        result.data.push(Bind {
            id: row.try_get::<u64, _>("id").map_err(Error::any)?.to_string(),
            group_id: row
                .try_get::<u64, _>("group_id")
                .map_err(Error::any)?
                .to_string(),
            bind_type: row.try_get("bind_type").map_err(Error::any)?,
            object_id: row
                .try_get::<u64, _>("object_id")
                .map_err(Error::any)?
                .to_string(),
            created_at: row.try_get("created_at").map_err(Error::any)?,
            updated_at: row.try_get("updated_at").map_err(Error::any)?,
        })
    }
    Ok(result)
}

pub struct QueryOpts {
    pub group_id: Vec<String>,
    pub bind_type: Option<u8>,
    pub object_id: Option<String>,
}

pub async fn list_all(pool: &MySqlPool, opts: &QueryOpts) -> Result<Vec<Bind>> {
    let mut wheres = String::from("");

    let length = opts.group_id.len();
    if length == 0 {
    } else if length == 1 {
        wheres.push_str(format!(r#"`group_id` = {}"#, opts.group_id[0]).as_str());
    } else {
        wheres.push_str(format!(r#"`group_id` IN ({})"#, opts.group_id.join(",")).as_str());
    }

    if let Some(bind_type) = opts.bind_type {
        if !wheres.is_empty() {
            wheres.push_str(" AND ");
        }
        wheres.push_str(format!(r#"`bind_type` = {}"#, bind_type).as_str());
    };
    if let Some(object_id) = &opts.object_id {
        if !wheres.is_empty() {
            wheres.push_str(" AND ");
        }
        wheres.push_str(format!(r#"`object_id` = {}"#, object_id).as_str());
    };
    if !wheres.is_empty() {
        wheres.push_str(" AND ");
    }

    let rows = sqlx::query(
        format!(
            r#"SELECT `id`,`group_id`,`bind_type`,`object_id`,`created_at`,`updated_at`
            FROM `bind`
            WHERE {} `deleted` = 0;"#,
            wheres,
        )
        .as_str(),
    )
    .fetch_all(pool)
    .await
    .map_err(Error::any)?;
    let mut result = Vec::with_capacity(rows.len());
    for row in rows.iter() {
        result.push(Bind {
            id: row.try_get::<u64, _>("id").map_err(Error::any)?.to_string(),
            group_id: row
                .try_get::<u64, _>("group_id")
                .map_err(Error::any)?
                .to_string(),
            bind_type: row.try_get("bind_type").map_err(Error::any)?,
            object_id: row
                .try_get::<u64, _>("object_id")
                .map_err(Error::any)?
                .to_string(),
            created_at: row.try_get("created_at").map_err(Error::any)?,
            updated_at: row.try_get("updated_at").map_err(Error::any)?,
        })
    }
    Ok(result)
}
