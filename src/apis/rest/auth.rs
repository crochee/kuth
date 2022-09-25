use axum::{
    extract::Query,
    headers::{
        authorization::{Basic, Bearer},
        Authorization, HeaderName,
    },
    Extension, Json, TypedHeader,
};
use http::{HeaderMap, HeaderValue, Request, StatusCode};
use sqlx::MySqlPool;

use crate::{
    service::{
        authentication::{self, basic, bearer},
        authorization::{Abac, Attribute, Matchers},
    },
    utils::valid::Valid,
    Error, Result,
};

pub async fn create_token(
    TypedHeader(Authorization(value)): TypedHeader<Authorization<Basic>>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<HeaderMap> {
    let (account_id, user_id, token) =
        basic::valid_sign(pool, value.username(), value.password()).await?;
    // 写入请求头
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-account-id"),
        HeaderValue::from_str(&account_id).map_err(Error::any)?,
    );
    headers.insert(
        HeaderName::from_static("x-user-id"),
        HeaderValue::from_str(&user_id).map_err(Error::any)?,
    );
    headers.insert(
        HeaderName::from_static("x-auth-token"),
        HeaderValue::from_str(&token).map_err(Error::any)?,
    );
    Ok(headers)
}

pub async fn verify_token(
    TypedHeader(Authorization(value)): TypedHeader<Authorization<Bearer>>,
    Valid(request_value): Valid<Query<authentication::Request>>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<Json<authentication::Effect>> {
    if request_value.resource.is_none() && request_value.path.is_none() {
        return Err(Error::BadRequest(
            "both resource and path are empty".to_owned(),
        ));
    }
    let resp = bearer::parse(pool, value.token(), &request_value).await?;
    Ok(resp.into())
}

pub async fn sign<T>(_req: Request<T>) -> Result<(StatusCode, String)>
where
    T: Send,
{
    Err(Error::Unauthorized("l".to_owned()))
}

pub async fn authorization(
    Valid(att): Valid<Json<Attribute>>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<()> {
    Abac.authorize(&pool, &att).await?;
    Ok(())
}
