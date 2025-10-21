use axum::{Extension, Json, http::header};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

use crate::entities::tasks;

pub async fn create_task(Extension(db): Extension<DatabaseConnection>) -> String {
    let new_task = tasks::ActiveModel {
        priority: Set(Some("High".to_owned())),
        title: Set("Finish writing Axum guide".to_owned()),
        description: Set(Some("Include SeaORM examples with Axum".to_owned())),
        user_id: Set(Some(1)), // assuming user ID 1 exists
        is_default: Set(Some(false)),
        ..Default::default()
    };

    let result = new_task.insert(&db).await;
    match result {
        Ok(task) => format!("✅ Task created with ID: {}", task.id),
        Err(err) => format!("❌ Failed to create task: {}", err),
    }
}
