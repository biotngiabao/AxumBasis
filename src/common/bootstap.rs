use crate::domain::auth::{domain::service::AuthService, infra::repo::AuthRepo};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use super::config;

pub struct AppState {
    pub config: config::Config,
    pub database: DatabaseConnection,
    pub auth_service: Arc<AuthService>,
}

impl AppState {
    pub fn build(config: config::Config, database: DatabaseConnection) -> Arc<Self> {
        let auth_repo = AuthRepo::new(database.clone());
        let auth_service = AuthService::new(auth_repo);

        Arc::new(AppState {
            config,
            database,
            auth_service,
        })
    }
}
