use chrono::Utc;
use serde::Deserialize;
use sqlx::{MySql, MySqlPool, Transaction};
use validator::Validate;

use crate::{
    model::{
        param::{check_effect, check_policy_type},
        List, Policy, ID,
    },
    utils::id::next_id,
    Error, Result,
};

#[derive(Debug, Deserialize, Validate)]
pub struct Content {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub desc: String,
    #[validate(length(min = 1))]
    pub version: String,
    #[validate(custom = "check_policy_type")]
    pub policy_type: u8,
    pub subjects: Vec<String>,
    #[validate(custom = "check_effect")]
    pub effect: String,
    pub action: Vec<String>,
    pub resources: Vec<String>,
    pub collections: Vec<String>,
}

pub async fn create(tx: &mut Transaction<'_, MySql>, content: &Content) -> Result<ID> {
    let policy_id = next_id().map_err(Error::any)?;
    let subjects = serde_json::to_string(&content.subjects).map_err(Error::any)?;
    let action = serde_json::to_string(&content.action).map_err(Error::any)?;
    let resources = serde_json::to_string(&content.resources).map_err(Error::any)?;
    let collections = serde_json::to_string(&content.collections).map_err(Error::any)?;
    sqlx::query!(
        r#"INSERT INTO `policy`
        (`id`,`name`,`desc`,`version`,`policy_type`,`subjects`,`effect`,`action`,`resources`,`collections`)
        VALUES(?,?,?,?,?,?,?,?,?,?);"#,
        policy_id,
        content.name,
        content.desc,
        content.version,
        content.policy_type,
        subjects,
        content.effect,
        action,
        resources,
        collections,
    )
    .execute(tx)
    .await
    .map_err(Error::any)?;
    Ok(ID {
        id: policy_id.to_string(),
    })
}

pub async fn insert_or_update(tx: &mut Transaction<'_, MySql>, content: &Content) -> Result<ID> {
    if let Some(v) = sqlx::query!(
        r#"SELECT `id` FROM `policy`
        WHERE `policy_type` = ? AND `version` = ? AND `desc` = ? AND `deleted` = 0 LIMIT 1;"#,
        content.policy_type,
        content.version,
        content.desc,
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(Error::any)?
    {
        let subjects = serde_json::to_string(&content.subjects).map_err(Error::any)?;
        let action = serde_json::to_string(&content.action).map_err(Error::any)?;
        let resources = serde_json::to_string(&content.resources).map_err(Error::any)?;
        let collections = serde_json::to_string(&content.collections).map_err(Error::any)?;
        sqlx::query!(
            r#"UPDATE `policy` SET
                `name` = ?,
                `subjects` = ?,
                `effect` = ?,
                `action` = ?,
                `resources` = ?,
                `collections` = ?
                WHERE `policy_type` = ? AND `version` = ? AND `desc` = ? AND `deleted` = 0;"#,
            content.name,
            subjects,
            content.effect,
            action,
            resources,
            collections,
            content.policy_type,
            content.version,
            content.desc,
        )
        .execute(tx)
        .await
        .map_err(Error::any)?;
        return Ok(ID {
            id: v.id.to_string(),
        });
    }
    create(tx, content).await
}

pub async fn delete(pool: &MySqlPool, id: &str) -> Result<()> {
    sqlx::query!(
        r#"UPDATE `policy` SET `deleted` = `id`,`deleted_at`= ? 
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
    #[validate(length(min = 1))]
    pub version: Option<String>,
    pub subjects: Option<Vec<String>>,
    #[validate(custom = "check_effect")]
    pub effect: Option<String>,
    pub action: Option<Vec<String>>,
    pub resources: Option<Vec<String>>,
    pub collections: Option<Vec<String>>,
}

pub async fn update(pool: &MySqlPool, id: &str, opts: &Opts) -> Result<()> {
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
    if let Some(version) = &opts.version {
        if !update_content.is_empty() {
            update_content.push_str(" , ");
        }
        update_content.push_str(format!(r#"`version` = '{}'"#, version).as_str());
    };
    if let Some(subjects) = &opts.subjects {
        let content = serde_json::to_string(subjects).map_err(Error::any)?;
        if !update_content.is_empty() {
            update_content.push_str(" , ");
        }
        update_content.push_str(format!(r#"`subjects` = '{}'"#, content).as_str());
    };
    if let Some(action) = &opts.action {
        let content = serde_json::to_string(action).map_err(Error::any)?;
        if !update_content.is_empty() {
            update_content.push_str(" , ");
        }
        update_content.push_str(format!(r#"`action` = '{}'"#, content).as_str());
    };
    if let Some(resources) = &opts.resources {
        let content = serde_json::to_string(resources).map_err(Error::any)?;
        if !update_content.is_empty() {
            update_content.push_str(" , ");
        }
        update_content.push_str(format!(r#"`resources` = '{}'"#, content).as_str());
    };
    if let Some(collections) = &opts.collections {
        let content = serde_json::to_string(collections).map_err(Error::any)?;
        if !update_content.is_empty() {
            update_content.push_str(" , ");
        }
        update_content.push_str(format!(r#"`collections` = '{}'"#, content).as_str());
    }

    if update_content.is_empty() {
        return Ok(());
    }
    sqlx::query(
        format!(
            r#"UPDATE `policy` SET {}
                WHERE `id` = ? AND `deleted` = 0;"#,
            update_content
        )
        .as_str(),
    )
    .bind(id)
    .execute(pool)
    .await
    .map_err(Error::any)?;
    Ok(())
}

pub async fn get(pool: &MySqlPool, id: &str) -> Result<Policy> {
    let row=match sqlx::query!(
        r#"SELECT `id`,`name`,`desc`,`version`,`policy_type`,`subjects`,`effect`,`action`,`resources`,`collections`,`created_at`,`updated_at`
        FROM `policy`
        WHERE `id` = ? AND `deleted` = 0;"#,
        id,
    )
    .fetch_optional(pool)
    .await
    {
        Ok(v) => match v {
            Some(value) => Ok(value),
            None => Err(Error::NotFound("no rows".to_owned())),
        },
        Err(err) => Err(Error::any(err)),
    }?;
    let subjects: Vec<String> = serde_json::from_str(&row.subjects).map_err(Error::any)?;
    let action: Vec<String> = serde_json::from_str(&row.action).map_err(Error::any)?;
    let resources: Vec<String> = serde_json::from_str(&row.resources).map_err(Error::any)?;
    let collections: Vec<String> = serde_json::from_str(&row.collections).map_err(Error::any)?;
    Ok(Policy {
        id: row.id.to_string(),
        name: row.name,
        desc: row.desc,
        version: row.version,
        policy_type: row.policy_type,
        subjects,
        effect: row.effect,
        action,
        resources,
        collections,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

pub async fn exist(tx: &mut Transaction<'_, MySql>, id: &str) -> Result<()> {
    let result = sqlx::query!(
        r#"SELECT COUNT(*) as count FROM `policy`
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

pub struct ListOpts {
    pub version: Option<String>,
    pub policy_type: Option<u8>,
    pub limit: u64,
    pub offset: u64,
    pub sort: String,
}

pub async fn list(pool: MySqlPool, opts: &ListOpts) -> Result<List<Policy>> {
    match &opts.version {
        Some(version) => match opts.policy_type {
            Some(policy_type) => {
                let policy_result = sqlx::query!(
                    r#"SELECT COUNT(`id`) as count FROM `policy`
                    WHERE `version` = ? AND `policy_type` = ? AND `deleted` = 0;"#,
                    version,
                    policy_type,
                )
                .fetch_one(&pool)
                .await
                .map_err(Error::any)?;
                let rows=sqlx::query!(
                    r#"SELECT `id`,`name`,`desc`,`version`,`policy_type`,`subjects`,`effect`,`action`,`resources`,`collections`,`created_at`,`updated_at`
                    FROM `policy`
                    WHERE `version` = ? AND `policy_type` = ? AND `deleted` = 0 ORDER BY ? LIMIT ? OFFSET ?;"#,
                    version,
                    policy_type,
                    opts.sort,
                    opts.limit,
                    opts.offset,
                )
                .fetch_all(&pool)
                .await
                .map_err(Error::any)?;
                let mut result = List {
                    data: Vec::new(),
                    limit: opts.limit,
                    offset: opts.offset,
                    total: policy_result.count,
                };
                for row in rows.iter() {
                    let subjects: Vec<String> =
                        serde_json::from_str(&row.subjects).map_err(Error::any)?;
                    let action: Vec<String> =
                        serde_json::from_str(&row.action).map_err(Error::any)?;
                    let resources: Vec<String> =
                        serde_json::from_str(&row.resources).map_err(Error::any)?;
                    let collections: Vec<String> =
                        serde_json::from_str(&row.collections).map_err(Error::any)?;
                    result.data.push(Policy {
                        id: row.id.to_string(),
                        name: row.name.clone(),
                        desc: row.desc.clone(),
                        version: row.version.clone(),
                        policy_type: row.policy_type,
                        subjects,
                        effect: row.effect.clone(),
                        action,
                        resources,
                        collections,
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                    });
                }
                Ok(result)
            }
            None => {
                let policy_result = sqlx::query!(
                    r#"SELECT COUNT(`id`) as count FROM `policy`
                    WHERE `version` = ? AND `deleted` = 0;"#,
                    version,
                )
                .fetch_one(&pool)
                .await
                .map_err(Error::any)?;
                let rows=sqlx::query!(
                    r#"SELECT `id`,`name`,`desc`,`version`,`policy_type`,`subjects`,`effect`,`action`,`resources`,`collections`,`created_at`,`updated_at`
                    FROM `policy`
                    WHERE `version` = ? AND `deleted` = 0 ORDER BY ? LIMIT ? OFFSET ?;"#,
                    version,
                    opts.sort,
                    opts.limit,
                    opts.offset,
                )
                .fetch_all(&pool)
                .await
                .map_err(Error::any)?;
                let mut result = List {
                    data: Vec::new(),
                    limit: opts.limit,
                    offset: opts.offset,
                    total: policy_result.count,
                };
                for row in rows.iter() {
                    let subjects: Vec<String> =
                        serde_json::from_str(&row.subjects).map_err(Error::any)?;
                    let action: Vec<String> =
                        serde_json::from_str(&row.action).map_err(Error::any)?;
                    let resources: Vec<String> =
                        serde_json::from_str(&row.resources).map_err(Error::any)?;
                    let collections: Vec<String> =
                        serde_json::from_str(&row.collections).map_err(Error::any)?;
                    result.data.push(Policy {
                        id: row.id.to_string(),
                        name: row.name.clone(),
                        desc: row.desc.clone(),
                        version: row.version.clone(),
                        policy_type: row.policy_type,
                        subjects,
                        effect: row.effect.clone(),
                        action,
                        resources,
                        collections,
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                    });
                }
                Ok(result)
            }
        },
        None => match opts.policy_type {
            Some(policy_type) => {
                let policy_result = sqlx::query!(
                    r#"SELECT COUNT(`id`) as count FROM `policy`
                    WHERE `policy_type` = ? AND `deleted` = 0;"#,
                    policy_type,
                )
                .fetch_one(&pool)
                .await
                .map_err(Error::any)?;
                let rows=sqlx::query!(
                    r#"SELECT `id`,`name`,`desc`,`version`,`policy_type`,`subjects`,`effect`,`action`,`resources`,`collections`,`created_at`,`updated_at`
                    FROM `policy`
                    WHERE `policy_type` = ? AND `deleted` = 0 ORDER BY ? LIMIT ? OFFSET ?;"#,
                    policy_type,
                    opts.sort,
                    opts.limit,
                    opts.offset,
                )
                .fetch_all(&pool)
                .await
                .map_err(Error::any)?;
                let mut result = List {
                    data: Vec::new(),
                    limit: opts.limit,
                    offset: opts.offset,
                    total: policy_result.count,
                };
                for row in rows.iter() {
                    let subjects: Vec<String> =
                        serde_json::from_str(&row.subjects).map_err(Error::any)?;
                    let action: Vec<String> =
                        serde_json::from_str(&row.action).map_err(Error::any)?;
                    let resources: Vec<String> =
                        serde_json::from_str(&row.resources).map_err(Error::any)?;
                    let collections: Vec<String> =
                        serde_json::from_str(&row.collections).map_err(Error::any)?;
                    result.data.push(Policy {
                        id: row.id.to_string(),
                        name: row.name.clone(),
                        desc: row.desc.clone(),
                        version: row.version.clone(),
                        policy_type: row.policy_type,
                        subjects,
                        effect: row.effect.clone(),
                        action,
                        resources,
                        collections,
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                    });
                }
                Ok(result)
            }
            None => {
                let policy_result = sqlx::query!(
                    r#"SELECT COUNT(`id`) as count FROM `policy`
                    WHERE `deleted` = 0;"#,
                )
                .fetch_one(&pool)
                .await
                .map_err(Error::any)?;
                let rows=sqlx::query!(
                    r#"SELECT `id`,`name`,`desc`,`version`,`policy_type`,`subjects`,`effect`,`action`,`resources`,`collections`,`created_at`,`updated_at`
                    FROM `policy`
                    WHERE `deleted` = 0 ORDER BY ? LIMIT ? OFFSET ?;"#,
                    opts.sort,
                    opts.limit,
                    opts.offset,
                )
                .fetch_all(&pool)
                .await
                .map_err(Error::any)?;
                let mut result = List {
                    data: Vec::new(),
                    limit: opts.limit,
                    offset: opts.offset,
                    total: policy_result.count,
                };
                for row in rows.iter() {
                    let subjects: Vec<String> =
                        serde_json::from_str(&row.subjects).map_err(Error::any)?;
                    let action: Vec<String> =
                        serde_json::from_str(&row.action).map_err(Error::any)?;
                    let resources: Vec<String> =
                        serde_json::from_str(&row.resources).map_err(Error::any)?;
                    let collections: Vec<String> =
                        serde_json::from_str(&row.collections).map_err(Error::any)?;
                    result.data.push(Policy {
                        id: row.id.to_string(),
                        name: row.name.clone(),
                        desc: row.desc.clone(),
                        version: row.version.clone(),
                        policy_type: row.policy_type,
                        subjects,
                        effect: row.effect.clone(),
                        action,
                        resources,
                        collections,
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                    });
                }
                Ok(result)
            }
        },
    }
}
