use sqlx::MySqlPool;

use crate::{
    service,
    store::{cache::Cache, mysql::group},
    Result,
};

pub async fn delete(pool: &MySqlPool, id: &str, account_id: &str) -> Result<()> {
    group::delete(pool, id, account_id).await?;
    let pairs = service::bind::get_id_pair_with_group(pool, id).await?;
    for (user_id, policy_id) in pairs {
        Cache.delete_policy(&user_id, &policy_id).await?;
    }
    Ok(())
}
