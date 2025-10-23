use crate::common::config;
use sea_orm::DatabaseConnection;

pub struct AppState {
    pub db: DatabaseConnection,
    pub config: config::Config,
}

impl AppState {
    pub fn new(db: DatabaseConnection, config: config::Config) -> Self {
        AppState { db, config }
    }
    pub fn build(db: DatabaseConnection, config: config::Config) -> AppState {
        AppState::new(db, config)
    }
}
