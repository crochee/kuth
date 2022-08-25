use std::collections::HashMap;

use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::Rng;
use sqlx::MySqlPool;

use crate::{
    model::cache::Password,
    store::{
        cache::{Cache, DEFAULT_EXPIRE_SEC},
        mysql::user::get_with_password,
    },
    utils::pw::verity_password,
    Error, Result,
};

pub async fn valid_sign(
    pool: MySqlPool,
    user: &str,
    password: &str,
) -> Result<(String, String, String)> {
    let value = get_password(pool, user).await?;
    if !verity_password(password, &value.password)? {
        return Err(Error::Forbidden("incorrect password".to_string()));
    }
    let token = sign(&value).await?;
    Ok((value.account_id, value.user_id, token))
}

async fn sign(p: &Password) -> Result<String> {
    let content = super::Claims {
        id: p.user_id.clone(),
        account_id: p.account_id.clone(),
        name: p.name.clone(),
        admin: p.admin,
        exp: Some(Utc::now().timestamp() as u64 + DEFAULT_EXPIRE_SEC as u64),
        args: p.extra.clone(),
    };
    // 生成随机密钥
    let secret_key = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(64)
        .map(char::from)
        .collect::<String>();
    // 将密钥存储到redis
    Cache.set_temp_secret(&content.id, &secret_key).await?;
    // 加密
    let token_header = Header {
        kid: Some(content.id.clone()),
        ..Default::default()
    };

    encode(
        &token_header,
        &content,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )
    .map_err(Error::any)
}

async fn get_password(pool: MySqlPool, user: &str) -> Result<Password> {
    match Cache.get_password(user).await {
        Ok(v) => Ok(v),
        Err(err) => {
            if Error::NotFound("".to_owned()).ne(&err) {
                return Err(err);
            }
            // 查询数据库
            let value = get_with_password(pool, user).await?;
            let mut result = Password {
                user_id: value.id,
                account_id: value.account_id,
                name: value.name,
                admin: value.admin,
                extra: HashMap::with_capacity(5),
                password: value.password,
            };
            result.extra.insert("desc".to_string(), vec![value.desc]);
            result
                .extra
                .insert("check".to_string(), vec![value.check.to_string()]);
            if let Some(email) = value.email {
                result.extra.insert("email".to_string(), vec![email]);
            }
            if let Some(sex) = value.sex {
                result.extra.insert("sex".to_string(), vec![sex]);
            }
            if let Some(image) = value.image {
                result.extra.insert("image".to_string(), vec![image]);
            }
            result
                .extra
                .insert("created_at".to_string(), vec![value.created_at.to_string()]);
            result
                .extra
                .insert("updated_at".to_string(), vec![value.updated_at.to_string()]);

            // 写缓存
            Cache.set_password(user, &result).await?;

            Ok(result)
        }
    }
}
