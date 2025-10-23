pub mod common;
pub mod domain;
mod router;

use dotenvy;
use sea_orm::DatabaseConnection;

mod app;
mod entities;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();

    let config = common::config::Config::from_env()?;
    let address: String = format!("{}:{}", config.server.host.as_str(), config.server.port);

    let db: DatabaseConnection = common::db::setup_database(&config).await?;
    let state = common::bootstap::AppState::build(config, db);
    let app = app::create_routers(state);

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(&address).await.unwrap();

    println!("Server is running on {address}");
    axum::serve(listener, app).await.unwrap();

    return Ok(());
}
