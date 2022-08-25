use sqlx::MySqlPool;

use crate::{
    service,
    store::{cache::Cache, mysql::policy},
    Result,
};

pub async fn delete(pool: MySqlPool, id: &str) -> Result<()> {
    policy::delete(&pool, id).await?;
    let pairs = service::bind::get_id_pair_with_policy(&pool, id).await?;
    for (user_id, policy_id) in pairs {
        Cache.delete_policy(&user_id, &policy_id).await?;
    }
    Ok(())
}
