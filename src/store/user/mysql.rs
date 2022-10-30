use chrono::Utc;

use crate::{
    model::{List, ID},
    store::pool::POOL,
    store::Store,
    utils::{id::next_id, pw::sign_password},
    Error, Result,
};

pub struct Mysql;

#[async_trait::async_trait]
impl<'a> Store<'a> for Mysql {
    type Output = super::User;
    type IDs = super::IDs;
    type Content = super::Content;
    type Opts = super::Opts;
    type Query = super::Query;

    async fn create(&self, content: &Self::Content) -> Result<ID> {
        let user_id = next_id().map_err(Error::any)?;
        let password = sign_password(&content.password)?;
        let pool = POOL.read().await.get_pool();
        sqlx::query!(
            r#"INSERT INTO `user`
        (`id`,`account_id`,`admin`,`name`,`desc`,`email`,`sex`,`image`,`password`)
        VALUES(?,?,?,?,?,?,?,?,?);"#,
            user_id,
            content.account_id,
            content.admin,
            content.name,
            content.desc,
            content.email,
            content.sex.map(|s| s.to_string()),
            content.image,
            password,
        )
        .execute(pool)
        .await
        .map_err(Error::any)?;
        Ok(ID {
            id: user_id.to_string(),
        })
    }
    async fn get(&self, id: &Self::IDs) -> Result<Self::Output> {
        let pool = POOL.read().await.get_pool();
        match sqlx::query!(
            r#"SELECT `id`,`account_id`,`admin`,`name`,`desc`,`email`,`check`,`sex`,`image`,`created_at`,`updated_at`
            FROM `user`
            WHERE `id` = ? AND `account_id` = ? AND `deleted` = 0;"#,
            id.id,
            id.account_id,
        )
        .map(|row|super::User {
            id: row.id.to_string(),
            account_id: row.account_id.to_string(),
            admin: row.admin!=0,
            name: row.name,
            desc: row.desc,
            email: row.email,
            check: row.check!=0,
            sex: row.sex,
            image: row.image,
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
            r#"UPDATE `user` SET `deleted` = `id`,`deleted_at`= ? 
            WHERE `id` = ? AND `account_id` = ? AND `deleted` = 0;"#,
            Some(Utc::now().naive_utc()),
            id.id,
            id.account_id,
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
        };
        if let Some(desc) = &opts.desc {
            if !update_content.is_empty() {
                update_content.push_str(" , ");
            }
            update_content.push_str(format!(r#"`desc` = '{}'"#, desc).as_str());
        };
        if let Some(email) = &opts.email {
            if !update_content.is_empty() {
                update_content.push_str(" , ");
            }
            update_content.push_str(format!(r#"`email` = '{}'"#, email).as_str());
        };
        if let Some(check) = &opts.check {
            if !update_content.is_empty() {
                update_content.push_str(" , ");
            }
            update_content.push_str(format!(r#"`check` = {}"#, check).as_str());
        };
        if let Some(sex) = &opts.sex {
            if !update_content.is_empty() {
                update_content.push_str(" , ");
            }
            update_content.push_str(format!(r#"`sex` = '{}'"#, sex.to_string()).as_str());
        };
        if let Some(image) = &opts.image {
            if !update_content.is_empty() {
                update_content.push_str(" , ");
            }
            update_content.push_str(format!(r#"`image` = '{}'"#, image).as_str());
        };
        if let Some(password) = &opts.password {
            let password_content = sign_password(password)?;
            if !update_content.is_empty() {
                update_content.push_str(" , ");
            }
            update_content.push_str(format!(r#"`password` = '{}'"#, password_content).as_str());
        }

        if update_content.is_empty() {
            return Ok(());
        }
        let pool = POOL.read().await.get_pool();
        sqlx::query(
            format!(
                r#"UPDATE `user` SET {}
                WHERE `id` = ? AND `account_id` = ? AND `deleted` = 0;"#,
                update_content
            )
            .as_str(),
        )
        .bind(id.id)
        .bind(id.account_id)
        .execute(pool)
        .await
        .map_err(Error::any)?;
        Ok(())
    }
    async fn list(&self, opts: &Self::Query) -> Result<List<Self::Output>> {
        let pool = POOL.read().await.get_pool();
        match &opts.account_id {
            Some(account_id) => {
                let user_result = sqlx::query!(
                    r#"SELECT COUNT(`id`) as count FROM `user`
                    WHERE `account_id` = ? AND `deleted` = 0;"#,
                    account_id,
                )
                .fetch_one(pool)
                .await
                .map_err(Error::any)?;
                let users = sqlx::query!(
                    r#"SELECT `id`,`account_id`,`admin`,`name`,`desc`,`email`,`check`,`sex`,`image`,`created_at`,`updated_at`
                    FROM `user`
                    WHERE `account_id` = ? AND `deleted` = 0 ORDER BY ? LIMIT ? OFFSET ?;"#,
                    account_id,
                    opts.sort,
                    opts.limit,
                    opts.offset,
                )
                .map(|row|{
                    super::User {
                        id: row.id.to_string(),
                        account_id: row.account_id.to_string(),
                        admin: row.admin!=0,
                        name: row.name,
                        desc: row.desc,
                        email: row.email,
                        check: row.check!=0,
                        sex: row.sex,
                        image: row.image,
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                    }
                })
                .fetch_all(pool)
                .await
                .map_err(Error::any)?;
                Ok(List {
                    data: users,
                    limit: opts.limit,
                    offset: opts.offset,
                    total: user_result.count,
                })
            }
            None => {
                let user_result = sqlx::query!(
                    r#"SELECT COUNT(`id`) as count FROM `user`
                    WHERE `deleted` = 0;"#,
                )
                .fetch_one(pool)
                .await
                .map_err(Error::any)?;
                let users = sqlx::query!(
                    r#"SELECT `id`,`account_id`,`admin`,`name`,`desc`,`email`,`check`,`sex`,`image`,`created_at`,`updated_at`
                    FROM `user`
                    WHERE `deleted` = 0 ORDER BY ? LIMIT ? OFFSET ?;"#,
                    opts.sort,
                    opts.limit,
                    opts.offset,
                )
                .map(|row|{
                   super:: User {
                        id: row.id.to_string(),
                        account_id: row.account_id.to_string(),
                        admin: row.admin!=0,
                        name: row.name,
                        desc: row.desc,
                        email: row.email,
                        check: row.check!=0,
                        sex: row.sex,
                        image: row.image,
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                    }
                })
                .fetch_all(pool)
                .await
                .map_err(Error::any)?;
                Ok(List {
                    data: users,
                    limit: opts.limit,
                    offset: opts.offset,
                    total: user_result.count,
                })
            }
        }
    }

    async fn exist(&self, id: &Self::IDs) -> Result<()> {
        let pool = POOL.read().await.get_pool();
        let result = sqlx::query!(
            r#"SELECT COUNT(*) as count FROM `user`
            WHERE `id` = ? AND `deleted` = 0 AND `account_id` = ? LIMIT 1;"#,
            id.id,
            id.account_id,
        )
        .fetch_one(pool)
        .await
        .map_err(Error::any)?;
        if result.count != 0 {
            return Ok(());
        }
        Err(Error::NotFound(format!("not found {}", id.id)))
    }
}
