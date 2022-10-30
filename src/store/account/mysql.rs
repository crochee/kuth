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
    type Output = super::Account;
    type IDs = String;
    type Content = super::Content;
    type Opts = super::Opts;
    type Query = super::Query;

    async fn create(&self, content: &Self::Content) -> Result<ID> {
        let account_id = next_id().map_err(Error::any)?;
        let pool = POOL.read().await.get_pool();
        sqlx::query!(
            r#"INSERT INTO `account`
            (`id`,`name`,`desc`)
            VALUES(?,?,?);"#,
            account_id,
            content.name,
            content.desc,
        )
        .execute(pool)
        .await
        .map_err(Error::any)?;
        Ok(ID {
            id: account_id.to_string(),
        })
    }
    async fn get(&self, id: &Self::IDs) -> Result<Self::Output> {
        let pool = POOL.read().await.get_pool();
        match sqlx::query!(
            r#"SELECT `id`,`name`,`desc`,`created_at`,`updated_at` FROM `account`
            WHERE `id` = ? AND `deleted` = 0;"#,
            id,
        )
        .map(|row| super::Account {
            id: row.id.to_string(),
            name: row.name,
            desc: row.desc,
            created_at: row.created_at,
            updated_at: row.updated_at,
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
    async fn delete(&self, id: &Self::IDs) -> Result<()> {
        let pool = POOL.read().await.get_pool();
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
    async fn update(&self, id: &Self::IDs, opts: &Self::Opts) -> Result<()> {
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
        let pool = POOL.read().await.get_pool();
        sqlx::query(
            format!(
                r#"UPDATE `account` SET {}
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
    async fn list(&self, opts: &Self::Query) -> Result<List<Self::Output>> {
        let pool = POOL.read().await.get_pool();

        let result = sqlx::query!(
            r#"SELECT COUNT(`id`) as count FROM `account`
                    WHERE `deleted` = 0;"#,
        )
        .fetch_one(pool)
        .await
        .map_err(Error::any)?;

        let rows = sqlx::query!(
            r#"SELECT `id`,`name`,`desc`,`created_at`,`updated_at`
                    FROM `policy`
                    WHERE `deleted` = 0 ORDER BY ? LIMIT ? OFFSET ?;"#,
            opts.sort,
            opts.limit,
            opts.offset,
        )
        .map(|row| super::Account {
            id: row.id.to_string(),
            name: row.name,
            desc: row.desc,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
        .fetch_all(pool)
        .await
        .map_err(Error::any)?;
        Ok(List {
            data: rows,
            limit: opts.limit,
            offset: opts.offset,
            total: result.count,
        })
    }
    async fn exist(&self, id: &Self::IDs) -> Result<()> {
        let pool = POOL.read().await.get_pool();
        let result = sqlx::query!(
            r#"SELECT COUNT(*) as count FROM `account`
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
