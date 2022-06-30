use std::error::Error;


mod routers;
mod app;
mod model;
mod config;
mod lib;

use lib::app as apps;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    apps::new_axum_server("./config/default.json").await?;
    Result::Ok(())
}
