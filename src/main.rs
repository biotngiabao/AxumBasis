mod middleware;
mod router;
mod dto;
pub mod domain;
pub mod common;

use dotenvy;
use sea_orm::{ DatabaseConnection };
mod entities;
mod app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();

    let config = common::config::Config::from_env()?;
    let db: DatabaseConnection = common::db::setup_database(&config).await?;
    let state = common::bootstap::AppState::build(config, db);
    let app = app::create_routers(state);
    let address: String = format!("0.0.0.0:{}", 8080);
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(&address).await.unwrap();

    println!("Server is running on {address}");
    axum::serve(listener, app).await.unwrap();

    return Ok(());
}
