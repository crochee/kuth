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
        param::{check_sort, transform_pagination, transform_sort},
        Group, List, ID,
    },
    service,
    store::mysql::group,
    utils::valid::{Header, Valid},
    Result,
};

pub async fn create(
    info: Header,
    Valid(mut content): Valid<Json<group::Content>>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<(StatusCode, Json<ID>)> {
    content.account_id = info.account_id;
    let resp = group::create(pool, &content).await?;
    Ok((StatusCode::CREATED, resp.into()))
}

pub async fn delete(
    info: Header,
    Path(id): Path<String>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<StatusCode> {
    service::group::delete(&pool, &id, &info.account_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn update(
    info: Header,
    Path(id): Path<String>,
    Valid(opts): Valid<Json<group::Opts>>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<StatusCode> {
    group::update(pool, &id, &info.account_id, &opts).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get(
    info: Header,
    Path(id): Path<String>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<Json<Group>> {
    let resp = group::get(pool, &id, &info.account_id).await?;
    Ok(resp.into())
}

#[derive(Debug, Deserialize, Validate)]
pub struct Filter {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    #[validate(custom = "check_sort")]
    pub sort: Option<String>,
}

pub async fn list(
    info: Header,
    Valid(filter): Valid<Query<Filter>>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<Json<List<Group>>> {
    let (limit, offset) = transform_pagination(filter.limit, filter.offset);
    let sort = transform_sort(&filter.sort);
    let resp = group::list(
        pool,
        &group::ListOpts {
            account_id: Some(info.account_id),
            limit,
            offset,
            sort,
        },
    )
    .await?;
    Ok(resp.into())
}
