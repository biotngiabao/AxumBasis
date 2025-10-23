use crate::dto::task_dto::*;
use crate::extractor::ValidatedJson;
use axum::{Extension, Json, extract::Path, http::StatusCode};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::{entities::tasks, response::ApiResponse};

pub async fn create_task(
    Extension(db): Extension<DatabaseConnection>,
    ValidatedJson(payload): ValidatedJson<TaskCreated>,
) -> String {
    let new_task = tasks::ActiveModel {
        priority: Set(payload.priority),
        title: Set(payload.title),
        description: Set(payload.description),
        user_id: Set(Some(payload.user_id)),
        is_default: Set(payload.is_default),
        ..Default::default()
    };

    let result = new_task.insert(&db).await;
    match result {
        Ok(task) => format!("✅ Task created with ID: {}", task.id),
        Err(err) => format!("❌ Failed to create task: {}", err),
    }
}

pub async fn get_task_by_id(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<ApiResponse<TaskResponse>>) {
    print!("Fetching task with ID: {}", id);
    match tasks::Entity::find_by_id(id).one(&db).await {
        Ok(Some(task)) => ApiResponse::success(
            TaskResponse {
                id: task.id,
                title: task.title,
                priority: task.priority,
                description: task.description,
                user_id: task.user_id.unwrap_or(0),
                is_default: task.is_default,
            },
            "Task retrieved successfully",
        ),

        Ok(None) => ApiResponse::error("Task not found", StatusCode::NOT_FOUND),
        Err(_) => ApiResponse::error("Internal server error", StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_all_tasks(
    Extension(db): Extension<DatabaseConnection>,
) -> (StatusCode, Json<ApiResponse<Vec<TaskResponse>>>) {
    match tasks::Entity::find().all(&db).await {
        Ok(task_list) => {
            let data: Vec<TaskResponse> = task_list
                .into_iter()
                .map(|task| TaskResponse {
                    id: task.id,
                    title: task.title,
                    priority: task.priority,
                    description: task.description,
                    user_id: task.user_id.unwrap_or(0),
                    is_default: task.is_default,
                })
                .collect();
            ApiResponse::success(data, "Tasks retrieved successfully")
        }
        Err(_) => ApiResponse::error("Internal server error", StatusCode::INTERNAL_SERVER_ERROR),
    }
}
