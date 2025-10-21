use std::sync::Arc;

use axum::{Extension, Router};
mod middleware;
mod router;
use dotenvy;
use sea_orm::{Database, DatabaseConnection};
mod entities;

pub struct AppState {
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let db = Database::connect(db_url).await.unwrap();

    //let state = Arc::new(db);
    let app: Router = router::create_routers(db);
    let address: String = format!("0.0.0.0:{}", 8080);
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(&address).await.unwrap();

    println!("Server is running on {address}");
    axum::serve(listener, app).await.unwrap();
}
