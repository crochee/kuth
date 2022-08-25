use axum::{extract::Path, Extension, Json};
use http::StatusCode;
use sqlx::MySqlPool;

use crate::{model::Account, service, store::mysql::account, utils::valid::Valid, Result};

pub async fn create(
    Valid(Json(mut content)): Valid<Json<service::account::Content>>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<(StatusCode, Json<service::account::Info>)> {
    let resp = service::account::create(pool, &mut content).await?;
    Ok((StatusCode::CREATED, resp.into()))
}

pub async fn delete(
    Path(id): Path<String>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<StatusCode> {
    service::account::delete(pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn update(
    Path(id): Path<String>,
    Valid(opts): Valid<Json<account::Opts>>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<StatusCode> {
    account::update(pool, &id, &opts).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get(
    Path(id): Path<String>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<Json<Account>> {
    let resp = account::get(pool, &id).await?;
    Ok(resp.into())
}
