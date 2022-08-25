use sqlx::MySqlPool;

use crate::{
    service,
    store::{cache::Cache, mysql::user},
    Result,
};

pub async fn update(pool: MySqlPool, id: &str, account_id: &str, opts: &user::Opts) -> Result<()> {
    user::update(pool, id, account_id, opts).await?;
    if opts.password.is_some() {
        Cache.delete_password(id).await?;
    }
    Ok(())
}

pub async fn delete(pool: &MySqlPool, id: &str, account_id: &str) -> Result<()> {
    let pairs = service::bind::get_id_pair_with_user(pool, id).await?;
    for (user_id, policy_id) in pairs {
        Cache.delete_policy(&user_id, &policy_id).await?;
    }
    user::delete(pool, id, account_id).await?;
    Cache.delete_password(id).await?;
    Ok(())
}
