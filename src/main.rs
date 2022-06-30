use std::{error::Error };


mod routers;
mod app;
mod model;
mod config;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    app::new_axum_server("./config/default.json").await?;
    Result::Ok(())
}
