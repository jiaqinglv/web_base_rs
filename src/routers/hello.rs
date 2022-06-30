use axum::{Router, routing::get};

use crate::app::api::hello;

pub fn new_router() -> axum::Router {
    // hello 路由
    let router = Router::new();
    // 版本控制
    let v1 = Router::new().
        route("/",  get(hello::hello_world)).
        route("/:name", get(hello::name));
    // 路由嵌套
    router.nest("/v1", v1)
}