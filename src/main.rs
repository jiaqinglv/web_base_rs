use tracing::{info, Level};
use tracing_subscriber::util::SubscriberInitExt;

mod routers;
mod app;

// 日志初始化
fn init_log(){
    tracing_subscriber::FmtSubscriber::builder()
    // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
    // will be written to stdout.
    .with_max_level(Level::DEBUG)
    // builds the subscriber.
    .finish().init();
}

// 创建axum http服务器
async fn new_axum_http_server(){
    let router = routers::new_http();
    info!("hettp server is running on http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    // 日志初始化
    init_log();
    // 初始化 axum http服务器
    new_axum_http_server().await
}
