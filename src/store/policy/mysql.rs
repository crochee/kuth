use chrono::Utc;

use crate::{
    model::{List, ID},
    store::pool::POOL,
    store::Store,
    utils::id::next_id,
    Error, Result,
};

pub struct Mysql;

#[async_trait::async_trait]
impl<'a> Store<'a> for Mysql {
    type Output = super::Policy;
    type IDs = String;
    type Content = super::Content;
    type Opts = String;
    type Query = super::Query;

    async fn create(&self, content: &Self::Content) -> Result<ID> {
        let policy_id = next_id().map_err(Error::any)?;
        let pool = POOL.read().await;
        sqlx::query!(
            r#"INSERT INTO `policy`
            (`id`,`name`,`scope`,`kind`,`desc`,`content`)
            VALUES(?,?,?,?,?,?);"#,
            policy_id,
            content.name,
            content.scope,
            content.kind.to_string(),
            content.desc,
            content.content,
        )
        .execute(pool.get_pool())
        .await
        .map_err(Error::any)?;
        Ok(ID {
            id: policy_id.to_string(),
        })
    }
    async fn get(&self, id: &Self::IDs) -> Result<Self::Output> {
        let pool = POOL.read().await;
        let row = match sqlx::query!(
            r#"SELECT `id`,`name`,`scope`,`kind`,`desc`,`content`,`created_at`,`updated_at`
            FROM `policy`
            WHERE `id` = ? AND `deleted` = 0;"#,
            id,
        )
        .fetch_optional(pool.get_pool())
        .await
        {
            Ok(v) => match v {
                Some(value) => Ok(value),
                None => Err(Error::NotFound("no rows".to_owned())),
            },
            Err(err) => Err(Error::any(err)),
        }?;
        Ok(super::Policy {
            id: row.id.to_string(),
            name: row.name,
            scope: row.scope,
            kind: row.kind.into(),
            desc: row.desc,
            content: row.content,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
    async fn delete(&self, id: &Self::IDs) -> Result<()> {
        let pool = POOL.read().await;
        sqlx::query!(
            r#"UPDATE `policy` SET `deleted` = `id`,`deleted_at`= ? 
            WHERE `id` = ? AND `deleted` = 0;"#,
            Some(Utc::now().naive_utc()),
            id,
        )
        .execute(pool.get_pool())
        .await
        .map_err(Error::any)?;
        Ok(())
    }
    async fn update(&self, id: &Self::IDs, content: &Self::Opts) -> Result<()> {
        let pool = POOL.read().await;
        sqlx::query!(
            r#"UPDATE `policy` SET `content`= ?
            WHERE `id` = ? AND `deleted` = 0;"#,
            content,
            id,
        )
        .execute(pool.get_pool())
        .await
        .map_err(Error::any)?;
        Ok(())
    }
    async fn list(&self, opts: &Self::Query) -> Result<List<Self::Output>> {
        let pool = POOL.read().await.get_pool();
        match &opts.kind {
            Some(v) => {
                let policy_result = sqlx::query!(
                    r#"SELECT COUNT(`id`) as count FROM `policy`
                    WHERE `kind` = ? AND `deleted` = 0;"#,
                    v.to_string(),
                )
                .fetch_one(pool)
                .await
                .map_err(Error::any)?;

                let rows = sqlx::query!(
                    r#"SELECT `id`,`name`,`scope`,`kind`,`desc`,`content`,`created_at`,`updated_at`
                    FROM `policy`
                    WHERE `kind` = ? AND `deleted` = 0 ORDER BY ? LIMIT ? OFFSET ?;"#,
                    v.to_string(),
                    opts.sort,
                    opts.limit,
                    opts.offset,
                )
                .map(|row| super::Policy {
                    id: row.id.to_string(),
                    name: row.name,
                    scope: row.scope,
                    kind: row.kind.into(),
                    desc: row.desc,
                    content: row.content,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                })
                .fetch_all(pool)
                .await
                .map_err(Error::any)?;
                return Ok(List {
                    data: rows,
                    limit: opts.limit,
                    offset: opts.offset,
                    total: policy_result.count,
                });
            }
            None => {
                let policy_result = sqlx::query!(
                    r#"SELECT COUNT(`id`) as count FROM `policy`
                    WHERE `deleted` = 0;"#,
                )
                .fetch_one(pool)
                .await
                .map_err(Error::any)?;

                let rows = sqlx::query!(
                    r#"SELECT `id`,`name`,`scope`,`kind`,`desc`,`content`,`created_at`,`updated_at`
                    FROM `policy`
                    WHERE `deleted` = 0 ORDER BY ? LIMIT ? OFFSET ?;"#,
                    opts.sort,
                    opts.limit,
                    opts.offset,
                )
                .map(|row| super::Policy {
                    id: row.id.to_string(),
                    name: row.name,
                    scope: row.scope,
                    kind: row.kind.into(),
                    desc: row.desc,
                    content: row.content,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                })
                .fetch_all(pool)
                .await
                .map_err(Error::any)?;
                return Ok(List {
                    data: rows,
                    limit: opts.limit,
                    offset: opts.offset,
                    total: policy_result.count,
                });
            }
        }
    }
    async fn exist(&self, id: &Self::IDs) -> Result<()>{
        let pool = POOL.read().await.get_pool();
        let result = sqlx::query!(
            r#"SELECT COUNT(*) as count FROM `policy`
            WHERE `id` = ? AND `deleted` = 0 LIMIT 1;"#,
            id,
        )
        .fetch_one(pool)
        .await
        .map_err(Error::any)?;
        if result.count != 0 {
            return Ok(());
        }
        Err(Error::NotFound(format!("not found {}", id)))
    }
}
