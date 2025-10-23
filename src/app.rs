use crate::common::bootstap::AppState;
use crate::domain::auth;
use axum::Router;
use std::sync::Arc;

pub fn create_routers(state: Arc<AppState>) -> Router {
    let cors = tower_http::cors::CorsLayer::new()
        .allow_headers(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any);

    let auth_router = Router::new().nest("/auth", auth::api::router::auth_handler());

    return Router::new()
        .merge(auth_router)
        .route("/", axum::routing::get(|| async { "Hello, World!" }))
        .layer(cors)
        .with_state(state);
}
