use super::handler;
use crate::common::bootstap::AppState;
use axum::{Router, routing};
use std::sync::Arc;

pub fn herax_handler() -> Router<Arc<AppState>> {
    return Router::new()
        .route("/ping", routing::get(handler::ping))
        .route("/server_info", routing::get(handler::get_server_info));
}
