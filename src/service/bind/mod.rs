use sqlx::MySqlPool;

use crate::{
    model::Bind,
    store::{cache::Cache, mysql::bind},
    Error, Result,
};

pub async fn delete(pool: MySqlPool, id: &str) -> Result<()> {
    let value = bind::get(&pool, id).await?;
    let pairs = get_id_pair_with_bind(&pool, &value).await?;
    bind::delete(&pool, id).await?;

    for (user_id, policy_id) in pairs {
        Cache.delete_policy(&user_id, &policy_id).await?;
    }
    Ok(())
}

pub async fn get_id_pair_with_user(pool: &MySqlPool, uid: &str) -> Result<Vec<(String, String)>> {
    // 根据user_id查出绑定的group
    let users = bind::list_all(
        pool,
        &bind::QueryOpts {
            group_id: vec![],
            bind_type: Some(1),
            object_id: Some(uid.to_owned()),
        },
    )
    .await?;
    let mut group_ids = Vec::with_capacity(users.len());
    for value in &users {
        group_ids.push(value.group_id.clone());
    }
    // 根据group查出policy
    let policys = bind::list_all(
        pool,
        &bind::QueryOpts {
            group_id: group_ids,
            bind_type: Some(2),
            object_id: None,
        },
    )
    .await?;
    let mut results = Vec::with_capacity(users.len() + policys.len());

    for value in users {
        for temp_value in &policys {
            if value.group_id.eq(&temp_value.group_id) {
                results.push((value.object_id.clone(), temp_value.object_id.clone()));
            }
        }
    }
    Ok(results)
}

pub async fn get_id_pair_with_group(pool: &MySqlPool, gid: &str) -> Result<Vec<(String, String)>> {
    // 根据group查询绑定关系
    let groups = bind::list_all(
        pool,
        &bind::QueryOpts {
            group_id: vec![gid.to_owned()],
            bind_type: None,
            object_id: None,
        },
    )
    .await?;
    let mut results = Vec::with_capacity(groups.len());

    for value in &groups {
        if value.bind_type == 1 {
            for temp_value in &groups {
                if temp_value.bind_type == 2 {
                    results.push((value.object_id.clone(), temp_value.object_id.clone()));
                }
            }
        }
    }
    Ok(results)
}

pub async fn get_id_pair_with_policy(pool: &MySqlPool, pid: &str) -> Result<Vec<(String, String)>> {
    // 根据policy查询出group
    let policys = bind::list_all(
        pool,
        &bind::QueryOpts {
            group_id: vec![],
            bind_type: Some(2),
            object_id: Some(pid.to_owned()),
        },
    )
    .await?;
    let mut group_ids = Vec::with_capacity(policys.len());
    for value in &policys {
        group_ids.push(value.group_id.clone());
    }
    // 根据group查出user
    let users = bind::list_all(
        pool,
        &bind::QueryOpts {
            group_id: group_ids,
            bind_type: Some(1),
            object_id: None,
        },
    )
    .await?;

    let mut results = Vec::with_capacity(policys.len() + users.len());

    for value in users {
        for temp_value in &policys {
            if value.group_id.eq(&temp_value.group_id) {
                results.push((value.object_id.clone(), temp_value.object_id.clone()));
            }
        }
    }
    Ok(results)
}

pub async fn get_id_pair_with_bind(
    pool: &MySqlPool,
    value: &Bind,
) -> Result<Vec<(String, String)>> {
    match value.bind_type {
        1 => {
            // 根据group查询policy
            let policys = bind::list_all(
                pool,
                &bind::QueryOpts {
                    group_id: vec![value.group_id.clone()],
                    bind_type: Some(2),
                    object_id: None,
                },
            )
            .await?;
            let mut results = Vec::with_capacity(policys.len());
            for temp_value in policys {
                results.push((value.object_id.clone(), temp_value.object_id.clone()));
            }
            Ok(results)
        }
        2 => {
            // 根据group查询user
            let users = bind::list_all(
                pool,
                &bind::QueryOpts {
                    group_id: vec![value.group_id.clone()],
                    bind_type: Some(1),
                    object_id: None,
                },
            )
            .await?;
            let mut results = Vec::with_capacity(users.len());
            for temp_value in users {
                results.push((temp_value.object_id.clone(), value.object_id.clone()));
            }
            Ok(results)
        }
        _ => Err(Error::BadRequest(format!(
            "bind_type is {}",
            value.bind_type
        ))),
    }
}
