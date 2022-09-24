use std::task::{Context, Poll};

use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use http::HeaderValue;
use sqlx::MySqlPool;
use tower::{Layer, Service};

use crate::{
    service::{
        authentication::{bearer, Effect, Request as ARequest},
        authorization::Decision,
    },
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
    if req.uri().path().ne("/v1/accounts") {
        // 获取token
        let value = match req
            .headers()
            .typed_try_get::<Authorization<Bearer>>()
            .map_err(|err| Error::Forbidden(err.to_string()))
        {
            Ok(v) => match v {
                Some(v) => v,
                None => {
                    return Error::Forbidden("miss request header Authorization".to_string())
                        .into_response()
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
                if Error::NotFound("".to_owned()).eq(&err) {
                    return Json(Effect {
                        decision: Decision::Deny.to_string(),
                        reason: err.to_string(),
                        user_id: Default::default(),
                        account_id: Default::default(),
                    })
                    .into_response();
                }
                return err.into_response();
            }
        };
        if !resp.decision.eq(&Decision::Allow.to_string()) {
            return Json(resp).into_response();
        }
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
    }
    next.run(req).await
}
