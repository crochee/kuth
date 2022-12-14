mod apis;
mod middlewares;
mod model;
mod routes;
mod service;
mod store;
mod utils;

pub use utils::e::Error;
pub type Result<T, E = Error> = core::result::Result<T, E>;
pub use routes::rest::api_rest_router;
pub use store::mysql::get_conn_pool;
