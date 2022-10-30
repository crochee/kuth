#![forbid(unsafe_code)]

use std::{io, net::SocketAddr, sync::Arc};

use axum::Server;
use tokio::runtime::Builder;
use tracing_subscriber::FmtSubscriber;

use kuth::{api_rest_router, init, Error, Result};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    FmtSubscriber::builder()
        .with_line_number(true)
        .with_max_level(tracing::Level::DEBUG)
        .with_level(true)
        .with_writer(io::stdout)
        .init();

    // tokio的运行时配置
    let raw_runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    let arc_runtime = Arc::new(raw_runtime);
    // 异步阻塞http服务
    arc_runtime.block_on(async {
        init().await?;
        let rest = api_rest_router(pool);

        Server::bind(&SocketAddr::from(([0, 0, 0, 0], 30050)))
            .http1_title_case_headers(true)
            .serve(rest.into_make_service())
            .with_graceful_shutdown(async move {
                let _ = tokio::signal::ctrl_c().await;
            })
            .await
            .map_err(Error::any)
    })
}
