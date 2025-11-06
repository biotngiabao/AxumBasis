use super::handler;
use crate::common::bootstap::AppState;
use axum::{Router, routing};
use std::sync::Arc;

pub fn task_handler() -> Router<Arc<AppState>> {
    return Router::new()
        .route("/create", routing::post(handler::create_task))
        .route("/all", routing::get(handler::get_all_tasks))
        // .route("/:id", routing::get(handler::get_task_by_id));
}
