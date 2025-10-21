use axum::{ routing, Extension, Router, middleware };
use tower_http::cors::{ self, Any };
use crate::router::draft::*;
use crate::middleware::log;
use crate::middleware::auth;

mod draft;
#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routers() -> Router {
    let cors: cors::CorsLayer = cors::CorsLayer::new().allow_headers(Any).allow_methods(Any);
    let shared_data = SharedData { message: "heheh".to_string() };

    return Router::new()
        .route("/", routing::get(helloworld))
        .route("/body", routing::post(get_body))
        .route("/body/json", routing::post(get_body_json))
        .route("/header", routing::get(get_header))
        .route("/share", routing::get(get_shared_data))
        .layer(cors)
        .layer(Extension(shared_data))
        .layer(middleware::from_fn(log::log_middleware))
        .layer(middleware::from_fn(auth::auth_middleware));
}
