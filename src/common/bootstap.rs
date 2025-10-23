use crate::domain::auth::{domain::service::AuthService, infra::repo::AuthRepo};
use crate::domain::task::domain::service::TaskService;
use crate::domain::task::infra::repo::TaskRepo;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use super::config;

pub struct AppState {
    pub config: config::Config,
    pub database: DatabaseConnection,
    pub auth_service: Arc<AuthService>,
    pub task_service: Arc<TaskService>,
}

impl AppState {
    pub fn build(config: config::Config, database: DatabaseConnection) -> Arc<Self> {
        let auth_repo = AuthRepo::new(database.clone());
        let auth_service = AuthService::new(auth_repo);

        let task_repo = TaskRepo::new(database.clone());
        let task_service = TaskService::new(task_repo);

        Arc::new(AppState {
            config,
            database,
            auth_service,
            task_service,
        })
    }
}
