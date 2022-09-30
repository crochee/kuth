use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use http::StatusCode;
use serde::Deserialize;
use sqlx::MySqlPool;
use validator::Validate;

use crate::{
    model::{
        param::{check_policy_type, check_sort, transform_pagination, transform_sort},
        List, Policy, ID,
    },
    service,
    store::mysql::policy,
    utils::valid::Valid,
    Error, Result,
};

pub async fn create(
    Valid(content): Valid<Json<policy::Content>>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<(StatusCode, Json<ID>)> {
    let mut tx = pool.begin().await.map_err(Error::any)?;
    let resp = policy::create(&mut tx, &content).await?;
    tx.commit().await.map_err(Error::any)?;
    Ok((StatusCode::CREATED, resp.into()))
}

pub async fn delete(
    Path(id): Path<String>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<StatusCode> {
    service::policy::delete(pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get(
    Path(id): Path<String>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<Json<Policy>> {
    let resp = policy::get(&pool, &id).await?;
    Ok(resp.into())
}

#[derive(Debug, Deserialize, Validate)]
pub struct Filter {
    #[validate(length(min = 1))]
    pub version: Option<String>,
    #[validate(custom = "check_policy_type")]
    pub policy_type: Option<u8>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    #[validate(custom = "check_sort")]
    pub sort: Option<String>,
}

pub async fn list(
    Valid(filter): Valid<Query<Filter>>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<Json<List<Policy>>> {
    let (limit, offset) = transform_pagination(filter.limit, filter.offset);
    let sort = transform_sort(&filter.sort);
    let resp = policy::list(
        pool,
        &policy::ListOpts {
            version: filter.version.clone(),
            policy_type: filter.policy_type,
            limit,
            offset,
            sort,
        },
    )
    .await?;
    Ok(resp.into())
}
