use super::handler;
use crate::common::bootstap::AppState;
use axum::{Router, routing};
use std::sync::Arc;

pub fn auth_handler() -> Router<Arc<AppState>> {
    return Router::new()
        .route("/login", routing::post(handler::login))
        .route("/register", routing::post(handler::register));
}
