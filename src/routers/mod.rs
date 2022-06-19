use axum::Router;
use tower_http::trace::TraceLayer;

mod hello;


pub fn new_http() -> Router {
    let router = Router::new();
    let hello_router = hello::new_http();
    router.nest("/api/", hello_router) .layer(TraceLayer::new_for_http())
}

