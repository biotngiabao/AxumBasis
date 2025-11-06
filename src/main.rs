pub mod common;
pub mod domain;

use std::sync::Arc;

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
    let grpc_client = domain::herax::infra::grpc_client::GprcClient::connect(
        config.grpc_server.host.clone(),
        config.grpc_server.port,
    )
    .await?;

    let state = common::bootstap::AppState::build(config, db, grpc_client);
    let app = app::create_routers(state);

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(&address).await.unwrap();

    println!("Server is running on {address}");
    axum::serve(listener, app).await.unwrap();

    return Ok(());
}
