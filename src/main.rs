use std::sync::Arc;

use axum::Router;
mod common;
mod dto;
pub mod extractor;
mod middleware;
pub mod response;
mod router;
use dotenvy;
use sea_orm::{Database, DatabaseConnection};

use crate::common::config;
mod entities;

pub struct AppState {
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = config::Config::from_env()?;
    let db = config::setup_database(&config).await?;

    //let state = Arc::new(db);
    let app: Router = router::create_routers(db);
    let address: String = format!("0.0.0.0:{}", 8080);
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(&address).await.unwrap();

    println!("Server is running on {address}");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
