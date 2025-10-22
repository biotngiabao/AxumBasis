use axum::{ routing, Router };
use crate::common::bootstap::AppState;
use super::handler;

pub fn auth_handler() -> Router<AppState> {
    return Router::new()
        .route("/login", routing::post(handler::login))
        .route("/register", routing::post(handler::register));
}
