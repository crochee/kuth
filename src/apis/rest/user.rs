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
        List, User, ID,
    },
    service,
    store::mysql::user,
    utils::valid::{Header, Valid},
    Error, Result,
};

pub async fn create(
    info: Header,
    Valid(mut content): Valid<Json<user::Content>>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<(StatusCode, Json<ID>)> {
    content.account_id = info.account_id;
    content.admin = 1;
    let mut tx = pool.begin().await.map_err(Error::any)?;
    let resp = user::create(&mut tx, &content).await?;
    tx.commit().await.map_err(Error::any)?;
    Ok((StatusCode::CREATED, resp.into()))
}

pub async fn delete(
    info: Header,
    Path(id): Path<String>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<StatusCode> {
    service::user::delete(&pool, &id, &info.account_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn update(
    info: Header,
    Path(id): Path<String>,
    Valid(opts): Valid<Json<user::Opts>>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<StatusCode> {
    service::user::update(pool, &id, &info.account_id, &opts).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get(
    info: Header,
    Path(id): Path<String>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<Json<User>> {
    let resp = user::get(pool, &id, &info.account_id).await?;
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
) -> Result<Json<List<User>>> {
    let (limit, offset) = transform_pagination(filter.limit, filter.offset);
    let sort = transform_sort(&filter.sort);
    let resp = user::list(
        pool,
        &user::ListOpts {
            account_id: Some(info.account_id),
            limit,
            offset,
            sort,
        },
    )
    .await?;
    Ok(resp.into())
}
