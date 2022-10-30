use std::{
    env,
    sync::{Arc},
};

use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use tokio::sync::RwLock;

use crate::{Error, Result};

pub struct MyPool(Option<MySqlPool>);

impl MyPool {
    async fn new(uri: &str, max_size: u32, min_idle: u32) -> Result<Self> {
        let pool = MySqlPoolOptions::new()
            .max_connections(max_size)
            .min_connections(min_idle)
            .connect(uri)
            .await
            .map_err(Error::any)?;
        Ok(Self(Some(pool)))
    }
    pub fn get_pool(&self) -> &MySqlPool {
        self.0.as_ref().unwrap()
    }
}

impl Default for MyPool {
    fn default() -> Self {
        Self(Default::default())
    }
}

lazy_static::lazy_static! {
    pub static ref POOL: Arc<RwLock<MyPool>> = Arc::new(RwLock::new(Default::default()));
}

pub async fn init() -> Result<()> {
    let max_size = env::var("CONNECTION_POOL_MAX_SIZE")
        .map(|s| str::parse::<u32>(&s).unwrap_or(50))
        .unwrap_or(50);
    let min_idle = env::var("CONNECTION_POOL_MIN_IDLE")
        .map(|s| str::parse::<u32>(&s).unwrap_or(30))
        .unwrap_or(30);
    let uri = env::var("DATABASE_URL").map_err(Error::any)?;
    let pool = POOL.clone();
    let mut pool = pool.write().await;
    *pool = MyPool::new(&uri, max_size, min_idle).await?;
    Ok(())
}
