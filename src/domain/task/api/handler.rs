use axum::extract::State;
use axum::{Json, extract::Path, http::StatusCode};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::common::bootstap::AppState;
use crate::common::response::ApiResponse;
use crate::entities::tasks;
use crate::{common::extractor::ValidatedJson, domain::task::dto::task_dto::*};
use std::sync::Arc;

pub async fn create_task(
    State(app_state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<TaskCreated>,
) -> (StatusCode, Json<ApiResponse<TaskResponse>>) {
    match app_state.task_service.create_task(payload).await {
        Ok(task) => ApiResponse::success(task, "Task created successfully"),
        Err(err) => ApiResponse::error(
            &format!("Failed to create task: {}", err),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}

pub async fn get_task_by_id(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<ApiResponse<TaskResponse>>) {
    match app_state.task_service.get_task_by_id(id).await {
        Ok(task) => ApiResponse::success(task, "Task retrieved successfully"),
        Err(err) => ApiResponse::error(
            &format!("Failed to fetch task: {}", err),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}

pub async fn get_all_tasks(
    State(app_state): State<Arc<AppState>>,
) -> (StatusCode, Json<ApiResponse<Vec<TaskResponse>>>) {
    match app_state.task_service.get_all_tasks().await {
        Ok(tasks) => ApiResponse::success(tasks, "Tasks retrieved successfully"),
        Err(err) => ApiResponse::error(
            &format!("Failed to fetch tasks: {}", err),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}
