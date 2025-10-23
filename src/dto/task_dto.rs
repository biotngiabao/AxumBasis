use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct TaskCreated {
    pub priority: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub user_id: i32,
    pub is_default: Option<bool>,
}

#[derive(Serialize)]
pub struct TaskResponse {
    pub id: i32,
    pub priority: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub user_id: i32,
    pub is_default: Option<bool>,
}
