use std::task::{Context, Poll};

use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use tower::{Layer, Service};

use crate::Error;

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

pub async fn check_headers<B>(req: Request<B>, next: Next<B>) -> Response {
    if req.uri().path().ne("/v1/accounts") {
        let header = req.headers();
        if header.get("X-Account-ID").is_none() {
            return Error::Forbidden("miss request header X-Account-ID".to_string())
                .into_response();
        };
        if header.get("X-User-ID").is_none() {
            return Error::Forbidden("miss request header X-User-ID".to_string()).into_response();
        };
    }
    next.run(req).await
}
