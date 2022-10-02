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
        List, Secret, ID,
    },
    store::mysql::secret,
    utils::valid::{Header, Valid},
    Error, Result,
};

pub async fn create(
    info: Header,
    Valid(mut content): Valid<Json<secret::Content>>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<(StatusCode, Json<ID>)> {
    content.user_id = info.user_id;
    let mut tx = pool.begin().await.map_err(Error::any)?;
    let resp = secret::create(&mut tx, &content).await?;
    tx.commit().await.map_err(Error::any)?;
    Ok((StatusCode::CREATED, resp.into()))
}

pub async fn delete(
    info: Header,
    Path(id): Path<String>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<StatusCode> {
    secret::delete(&pool, &id, &info.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize, Validate)]
pub struct Opts {
    #[validate(length(min = 1))]
    pub name: Option<String>,
}

pub async fn update(
    info: Header,
    Valid(opts): Valid<Json<Opts>>,
    Path(id): Path<String>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<StatusCode> {
    secret::update(pool, &id, &info.user_id, &opts.name).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get(
    info: Header,
    Path(id): Path<String>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<Json<Secret>> {
    let resp = secret::get(pool, &id, &info.user_id).await?;
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
) -> Result<Json<List<Secret>>> {
    let (limit, offset) = transform_pagination(filter.limit, filter.offset);
    let sort = transform_sort(&filter.sort);
    let resp = secret::list(
        &pool,
        &secret::ListOpts {
            user_id: Some(info.user_id),
            limit,
            offset,
            sort,
        },
    )
    .await?;
    Ok(resp.into())
}
