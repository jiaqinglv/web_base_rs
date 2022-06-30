use axum::{Router};
use tower_http::trace::TraceLayer;

mod hello;


pub fn new_http() -> Router {
    let router = Router::new();
    // router.route("/favicon.ico", get());
    let hello_router = hello::new_router();
    router.nest("/api/", hello_router) .layer(TraceLayer::new_for_http())
}

