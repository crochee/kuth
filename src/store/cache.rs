use std::str::from_utf8;

use redis::{AsyncCommands, ErrorKind, FromRedisValue, RedisError, RedisResult, Value};

use crate::{
    model::cache::{Password, Policy},
    Error, Result,
};

pub struct NilString {
    pub value: Option<String>,
}

impl FromRedisValue for NilString {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        match *v {
            Value::Data(ref bytes) => Ok(Self {
                value: Some(from_utf8(bytes)?.to_string()),
            }),
            Value::Okay => Ok(Self {
                value: Some("OK".to_string()),
            }),
            Value::Status(ref val) => Ok(Self {
                value: Some(val.to_string()),
            }),
            Value::Nil => Ok(Self { value: None }),
            _ => Err(RedisError::from((
                ErrorKind::TypeError,
                "Response was of incompatible type",
                format!("(response was {:?})", v),
            ))),
        }
    }
}

lazy_static::lazy_static! {
    pub static ref CLIENT: redis::Client = {
        redis::Client::open(std::env::var("REDIS_URL").unwrap()).unwrap()
    };
}

// half hour
pub const DEFAULT_EXPIRE_SEC: usize = 1800;

// pub struct Cache;

// impl Cache {
//     pub async fn set_kv(&self, key: &str, value: &str, seconds: usize) -> Result<()> {
//         let mut conn = CLIENT.get_async_connection().await.map_err(Error::any)?;
//         conn.set_ex(key, value, seconds).await.map_err(Error::any)
//     }
//     pub async fn get_kv(&self, key: &str) -> Result<String> {
//         let mut conn = CLIENT.get_async_connection().await.map_err(Error::any)?;
//         let value: NilString = conn.get(key).await.map_err(Error::any)?;
//         if value.0.is_empty() {
//             return Err(Error::NotFound(format!("invalid key {}", key)));
//         }
//         Ok(value.0)
//     }
//     pub async fn delete(&self, key: &str) -> Result<()> {
//         let mut conn = CLIENT.get_async_connection().await.map_err(Error::any)?;
//         conn.del(key).await.map_err(Error::any)
//     }

//     pub async fn set_policy(&self, uid: &str, value: &Policy) -> Result<()> {
//         let result = serde_json::to_string(value).map_err(Error::any)?;
//         self.set_kv(
//             format!("kuth:policy:{}:{}", uid, value.id).as_str(),
//             &result,
//             DEFAULT_EXPIRE_SEC,
//         )
//         .await
//     }
//     pub async fn delete_policy(&self, uid: &str, policy_id: &str) -> Result<()> {
//         self.delete(format!("kuth:policy:{}:{}", uid, policy_id).as_str())
//             .await
//     }
//     pub async fn get_policy(&self, uid: &str, policy_id: &str) -> Result<Policy> {
//         let result = self
//             .get_kv(format!("kuth:policy:{}:{}", uid, policy_id).as_str())
//             .await?;
//         serde_json::from_str(&result).map_err(Error::any)
//     }
//     pub async fn set_temp_secret(&self, uid: &str, value: &str) -> Result<()> {
//         self.set_kv(
//             format!("kuth:temp_secret:{}", uid).as_str(),
//             value,
//             DEFAULT_EXPIRE_SEC,
//         )
//         .await
//     }
//     pub async fn get_temp_secret(&self, uid: &str) -> Result<String> {
//         self.get_kv(format!("kuth:temp_secret:{}", uid).as_str())
//             .await
//             .map_err(|err| Error::Forbidden(err.to_string()))
//     }

//     pub async fn set_password(&self, uid: &str, value: &Password) -> Result<()> {
//         let temp_value = serde_json::to_string(value).map_err(Error::any)?;
//         self.set_kv(
//             format!("kuth:password:{}", uid).as_str(),
//             &temp_value,
//             DEFAULT_EXPIRE_SEC,
//         )
//         .await
//     }
//     pub async fn get_password(&self, uid: &str) -> Result<Password> {
//         let value = self
//             .get_kv(format!("kuth:password:{}", uid).as_str())
//             .await?;
//         serde_json::from_str(&value).map_err(Error::any)
//     }
//     pub async fn delete_password(&self, uid: &str) -> Result<()> {
//         self.delete(format!("kuth:password:{}", uid).as_str()).await
//     }
// }
