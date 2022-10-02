use std::time::Duration;

use axum::{
    extract::Extension,
    handler::Handler,
    headers::HeaderName,
    http::Uri,
    middleware,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Router,
};
use sqlx::MySqlPool;
use tower_http::{
    cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer, ExposeHeaders},
    trace::{DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;

use crate::{
    apis::rest::{account, auth, bind, group, policy, secret, user},
    middlewares::{check_headers, MakeSpanWithTrace, Trace},
    Error,
};

pub fn api_rest_router(pool: MySqlPool) -> Router {
    Router::new()
        // 系统接口
        .merge(route_v1())
        // 认证接口
        .merge(authz_api())
        .layer(Extension(pool))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(MakeSpanWithTrace::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
        .layer(Trace)
        .fallback(not_found.into_service())
        .layer(
            CorsLayer::new()
                .expose_headers(ExposeHeaders::list(vec![
                    HeaderName::from_static("x-auth-token"),
                    HeaderName::from_static("x-account-id"),
                    HeaderName::from_static("x-user-id"),
                ]))
                .allow_headers(AllowHeaders::mirror_request())
                .allow_methods(AllowMethods::mirror_request())
                .allow_origin(AllowOrigin::mirror_request())
                .allow_credentials(true)
                .max_age(Duration::from_secs(60) * 60 * 12),
        )
}

async fn not_found(uri: Uri) -> impl IntoResponse {
    Error::NotFound(format!("no route for {}", uri))
}

fn route_v1() -> Router {
    Router::new().nest("/v1", apis())
}

fn apis() -> Router {
    Router::new()
        .merge(account_api())
        .merge(user_api())
        .merge(secret_api())
        .merge(group_api())
        .layer(middleware::from_fn(check_headers))
        .merge(bind_api())
        .merge(policy_api())
}

// 账户操作
fn account_api() -> Router {
    Router::new()
        .route("/accounts", post(account::create))
        .route("/accounts/:id", delete(account::delete))
        .route("/accounts/:id", patch(account::update))
        .route("/accounts/:id", get(account::get))
}

// 用户操作
fn user_api() -> Router {
    Router::new()
        .route("/users", post(user::create))
        .route("/users/:id", delete(user::delete))
        .route("/users/:id", patch(user::update))
        .route("/users/:id", get(user::get))
        .route("/users", get(user::list))
}

// 密钥操作
fn secret_api() -> Router {
    Router::new()
        .route("/secrets", post(secret::create))
        .route("/secrets/:id", delete(secret::delete))
        .route("/secrets/:id", patch(secret::update))
        .route("/secrets/:id", get(secret::get))
        .route("/secrets", get(secret::list))
}

// 用户组操作
fn group_api() -> Router {
    Router::new()
        .route("/groups", post(group::create))
        .route("/groups/:id", delete(group::delete))
        .route("/groups/:id", patch(group::update))
        .route("/groups/:id", get(group::get))
        .route("/groups", get(group::list))
}

// 策略操作
fn policy_api() -> Router {
    Router::new()
        .route("/policies", post(policy::create))
        .route("/policies/:id", delete(policy::delete))
        .route("/policies/:id", patch(policy::update))
        .route("/policies/:id", get(policy::get))
        .route("/policies", get(policy::list))
}

// 用户组绑定和解绑用户
fn bind_api() -> Router {
    Router::new()
        .route("/binds", post(bind::create))
        .route("/binds/:id", delete(bind::delete))
        .route("/binds", get(bind::list))
}

// 认证
fn authz_api() -> Router {
    Router::new()
        .route("/v1/auth/tokens", post(auth::create_token))
        .route("/v1/auth", post(auth::verify_token))
        .route("/v1/signs", post(auth::sign))
        .route("/v1/authz", post(auth::authorization))
}
