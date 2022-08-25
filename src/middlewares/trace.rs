use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::ready;
use http::{HeaderValue, Request, Response};
use pin_project_lite::pin_project;
use tower::{Layer, Service};

use crate::utils::id::get_unique_id_string;

#[derive(Clone)]
pub struct Trace;

impl<S> Layer<S> for Trace {
    type Service = TraceService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TraceService { inner }
    }
}

#[derive(Clone)]
pub struct TraceService<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for TraceService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
    ResBody: Default,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let (mut head, body) = req.into_parts();
        let unique_id = match head.headers.get("X-Trace-Id") {
            Some(v) => v.clone(),
            None => HeaderValue::from_bytes(get_unique_id_string().as_bytes()).unwrap(),
        };
        // 请求头增加trace_id
        (&mut head.headers)
            .entry("X-Trace-Id")
            .or_insert(unique_id.clone());
        ResponseFuture {
            inner: self.inner.call(Request::from_parts(head, body)),
            trace_id: unique_id,
        }
    }
}

pin_project! {
    /// Response future for [`Trace`].
    ///
    /// [`Trace`]: super::Trace
    pub struct ResponseFuture<F> {
        #[pin]
         inner: F,
         trace_id:HeaderValue,
    }
}

impl<F, B, E> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response<B>, E>>,
{
    type Output = Result<Response<B>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let trace_id = this.trace_id;
        match ready!(this.inner.poll(cx)) {
            Ok(mut response) => {
                response
                    .headers_mut()
                    .insert("X-Trace-Id", trace_id.clone());
                Poll::Ready(Ok(response))
            }
            Err(err) => Poll::Ready(Err(err)),
        }
    }
}
