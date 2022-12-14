use std::task::{Context, Poll};

use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use http::{header, HeaderValue, Method};
use sqlx::MySqlPool;
use tower::{Layer, Service};

use crate::{
    service::authentication::{bearer, Request as ARequest},
    Error,
};

pub struct CheckHeader;

impl<S> Layer<S> for CheckHeader {
    type Service = CheckHeaderService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CheckHeaderService { inner }
    }
}

#[derive(Clone)]
pub struct CheckHeaderService<S> {
    inner: S,
}

impl<S, B> Service<Request<B>> for CheckHeaderService<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        self.inner.call(req)
    }
}

pub async fn check_headers<B>(mut req: Request<B>, next: Next<B>) -> Response {
    // 创建账户白名单
    if req.uri().path().eq("/accounts") && req.method().eq(&Method::POST) {
        return next.run(req).await;
    }
    // 获取token
    let value = match req
        .headers()
        .typed_try_get::<Authorization<Bearer>>()
        .map_err(|err| Error::Forbidden(err.to_string()))
    {
        Ok(v) => match v {
            Some(v) => v,
            None => {
                let mut resp = Error::Unauthorized("miss request header Authorization".to_string())
                    .into_response();
                resp.headers_mut().insert(
                    header::WWW_AUTHENTICATE,
                    HeaderValue::from_static("Bearer realm=Authorization Required"),
                );
                return resp;
            }
        },
        Err(err) => return err.into_response(),
    };
    // 获取连接
    let pool=  match req
            .extensions()
            .get::<MySqlPool>()
            .ok_or_else(|| {
                Error::Forbidden(format!(
                    "Extension of type `{}` was not found. Perhaps you forgot to add it? See `axum::Extension`.",
                    std::any::type_name::<MySqlPool>()
                ))
            })
            .map(|x| x.clone()){
                Ok(v) => v,
                Err(err) => return err.into_response(),
            };
    // 校验token
    let resp = match bearer::parse(
        pool,
        value.token(),
        &ARequest {
            resource: None,
            sub_resource: None,
            path: Some(req.uri().path().to_owned()),
            action: req.method().to_string(),
        },
    )
    .await
    {
        Ok(v) => v,
        Err(err) => {
            return err.into_response();
        }
    };
    // 重新赋值请求头
    let header = req.headers_mut();
    match HeaderValue::from_str(&resp.account_id) {
        Ok(v) => header.insert("X-Account-ID", v),
        Err(err) => return Error::any(err).into_response(),
    };
    match HeaderValue::from_str(&resp.user_id) {
        Ok(v) => header.insert("X-User-ID", v),
        Err(err) => return Error::any(err).into_response(),
    };
    next.run(req).await
}
