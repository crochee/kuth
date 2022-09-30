use chrono::Utc;
use serde::Deserialize;
use sqlx::{MySql, MySqlPool, Transaction};
use validator::Validate;

use crate::{
    model::{
        param::{check_check, check_password, check_sex},
        List, User, UserAll, ID,
    },
    utils::{id::next_id, pw::sign_password},
    Error, Result,
};

fn default_admin() -> u8 {
    2
}

#[derive(Debug, Deserialize, Validate)]
pub struct Content {
    #[serde(skip)]
    pub account_id: String,
    #[serde(skip, default = "default_admin")]
    #[validate(range(min = 1, max = 2))]
    pub admin: u8,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(custom = "check_password")]
    pub password: String,
    #[validate(length(min = 1))]
    pub desc: String,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(custom = "check_sex")]
    pub sex: Option<String>,
    #[validate(url)]
    pub image: Option<String>,
}

pub async fn create(tx: &mut Transaction<'_, MySql>, content: &Content) -> Result<ID> {
    super::account::exist(tx, &content.account_id).await?;
    let user_id = next_id().map_err(Error::any)?;
    let password = sign_password(&content.password)?;
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
        content.sex,
        content.image,
        password,
    )
    .execute(tx)
    .await
    .map_err(Error::any)?;
    Ok(ID {
        id: user_id.to_string(),
    })
}

pub async fn delete(pool: &MySqlPool, id: &str, account_id: &str) -> Result<()> {
    sqlx::query!(
        r#"UPDATE `user` SET `deleted` = `id`,`deleted_at`= ? 
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
    #[validate(custom = "check_password")]
    pub password: Option<String>,
    #[validate(length(min = 1))]
    pub desc: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(custom = "check_check")]
    pub check: Option<u8>,
    #[validate(custom = "check_sex")]
    pub sex: Option<String>,
    #[validate(url)]
    pub image: Option<String>,
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
        update_content.push_str(format!(r#"`sex` = '{}'"#, sex).as_str());
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
    sqlx::query(
        format!(
            r#"UPDATE `user` SET {}
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

pub async fn get(pool: MySqlPool, id: &str, account_id: &str) -> Result<User> {
    match sqlx::query!(
        r#"SELECT `id`,`account_id`,`admin`,`name`,`desc`,`email`,`check`,`sex`,`image`,`created_at`,`updated_at`
        FROM `user`
        WHERE `id` = ? AND `account_id` = ? AND `deleted` = 0;"#,
        id,
        account_id,
    )
    .map(|row|User {
        id: row.id.to_string(),
        account_id: row.account_id.to_string(),
        admin: row.admin,
        name: row.name,
        desc: row.desc,
        email: row.email,
        check: row.check,
        sex: row.sex,
        image: row.image,
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

pub async fn get_with_password(pool: MySqlPool, id: &str) -> Result<UserAll> {
    match sqlx::query!(
        r#"SELECT `id`,`account_id`,`admin`,`name`,`desc`,`email`,`check`,`sex`,`image`,`password`,`created_at`,`updated_at`
        FROM `user`
        WHERE `id` = ? AND `deleted` = 0;"#,
        id,
    )
    .map(|row|UserAll {
        id: row.id.to_string(),
        account_id: row.account_id.to_string(),
        admin: row.admin,
        name: row.name,
        desc: row.desc,
        email: row.email,
        check: row.check,
        sex: row.sex,
        image: row.image,
        password: row.password,
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

pub async fn list(pool: MySqlPool, opts: &ListOpts) -> Result<List<User>> {
    match &opts.account_id {
        Some(account_id) => {
            let user_result = sqlx::query!(
                r#"SELECT COUNT(`id`) as count FROM `user`
                WHERE `account_id` = ? AND `deleted` = 0;"#,
                account_id,
            )
            .fetch_one(&pool)
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
                User {
                    id: row.id.to_string(),
                    account_id: row.account_id.to_string(),
                    admin: row.admin,
                    name: row.name,
                    desc: row.desc,
                    email: row.email,
                    check: row.check,
                    sex: row.sex,
                    image: row.image,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                }
            })
            .fetch_all(&pool)
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
            .fetch_one(&pool)
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
                User {
                    id: row.id.to_string(),
                    account_id: row.account_id.to_string(),
                    admin: row.admin,
                    name: row.name,
                    desc: row.desc,
                    email: row.email,
                    check: row.check,
                    sex: row.sex,
                    image: row.image,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                }
            })
            .fetch_all(&pool)
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

pub async fn exist(tx: &mut Transaction<'_, MySql>, id: &str) -> Result<()> {
    let result = sqlx::query!(
        r#"SELECT COUNT(*) as count FROM `user`
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

pub async fn list_all_by_account_id(pool: &MySqlPool, account_id: &str) -> Result<Vec<User>> {
    sqlx::query!(
        r#"SELECT `id`,`account_id`,`admin`,`name`,`desc`,`email`,`check`,`sex`,`image`,`created_at`,`updated_at`
        FROM `user`
        WHERE `account_id` = ? AND `deleted` = 0;"#,
        account_id,
    )
    .map(|row|{
        User {
            id: row.id.to_string(),
            account_id: row.account_id.to_string(),
            admin: row.admin,
            name: row.name,
            desc: row.desc,
            email: row.email,
            check: row.check,
            sex: row.sex,
            image: row.image,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    })
    .fetch_all(pool)
    .await
    .map_err(Error::any)
}
