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
        param::{check_bind_type, check_sort, transform_pagination, transform_sort},
        Bind, List, ID,
    },
    service,
    store::mysql::bind,
    utils::valid::Valid,
    Result,
};

pub async fn create(
    Valid(content): Valid<Json<bind::Content>>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<(StatusCode, Json<ID>)> {
    let resp = bind::create(pool, &content).await?;
    Ok((StatusCode::CREATED, resp.into()))
}

pub async fn delete(
    Path(id): Path<String>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<StatusCode> {
    service::bind::delete(pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize, Validate)]
pub struct Filter {
    #[validate(length(min = 1))]
    pub group_id: Option<String>,
    #[validate(custom = "check_bind_type")]
    pub bind_type: Option<u8>,
    #[validate(length(min = 1))]
    pub object_id: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    #[validate(custom = "check_sort")]
    pub sort: Option<String>,
}

pub async fn list(
    Valid(filter): Valid<Query<Filter>>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<Json<List<Bind>>> {
    let (limit, offset) = transform_pagination(filter.limit, filter.offset);
    let sort = transform_sort(&filter.sort);
    let resp = bind::list(
        pool,
        &bind::ListOpts {
            group_id: filter.group_id.clone(),
            bind_type: filter.bind_type,
            object_id: filter.object_id.clone(),
            limit,
            offset,
            sort,
        },
    )
    .await?;
    Ok(resp.into())
}
