pub mod account;
pub mod bind;
pub mod group;
pub mod policy;
pub mod secret;
pub mod user;

use std::env;

use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

use crate::{Error, Result};

pub async fn get_conn_pool() -> Result<MySqlPool> {
    let max_size = env::var("CONNECTION_POOL_MAX_SIZE")
        .map(|s| str::parse::<u32>(&s).unwrap_or(50))
        .unwrap_or(50);
    let min_idle = env::var("CONNECTION_POOL_MIN_IDLE")
        .map(|s| str::parse::<u32>(&s).unwrap_or(30))
        .unwrap_or(30);
    let database_url = env::var("DATABASE_URL").map_err(Error::any)?;
    MySqlPoolOptions::new()
        .max_connections(max_size)
        .min_connections(min_idle)
        .connect(&database_url)
        .await
        .map_err(Error::any)
}
