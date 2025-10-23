use crate::middleware::auth;
use crate::middleware::log;
use crate::router::auth_router::*;
use crate::router::draft_router::*;
use crate::router::task_router::*;
use axum::{Extension, Router, middleware, routing};
use sea_orm::DatabaseConnection;
use tower_http::cors::{self, Any};

mod auth_router;
mod draft_router;
mod task_router;

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routers(db: DatabaseConnection) -> Router {
    let cors: cors::CorsLayer = cors::CorsLayer::new().allow_headers(Any).allow_methods(Any);
    let shared_data = SharedData {
        message: "heheh".to_string(),
    };

    return Router::new()
        .route("/", routing::get(helloworld))
        .route("/body", routing::post(get_body))
        .route("/body/json", routing::post(get_body_json))
        .route("/header", routing::get(get_header))
        .route("/share", routing::get(get_shared_data))
        .route("/auth/register", routing::post(register))
        .route("/task", routing::post(create_task))
        .route("/task/{id}", routing::get(get_task_by_id))
        .route("/task", routing::get(get_all_tasks))
        .route("/auth/login", routing::post(login))
        .layer(cors)
        .layer(Extension(shared_data))
        .layer(Extension(db))
        .layer(middleware::from_fn(log::log_middleware));
    //.layer(middleware::from_fn(auth::auth_middleware));
}
