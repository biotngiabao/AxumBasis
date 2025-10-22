use sea_orm::{ DatabaseConnection };

use crate::domain::auth::domain::service::AuthService;

use super::config;

#[derive(Clone)]
pub struct AppState {
    pub config: config::Config,
    pub database: DatabaseConnection,
    pub auth_service: AuthService,
}

impl AppState {
    pub fn build(config: config::Config, database: DatabaseConnection) -> Self {
        let auth_service = AuthService {
            db: database.clone(),
        };
        AppState { config, database, auth_service }
    }
}
