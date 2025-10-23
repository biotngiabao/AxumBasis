use axum::{Json, extract::State, http::StatusCode};

use crate::common::bootstap::AppState;
use crate::common::extractor::ValidatedJson;
use crate::common::response::ApiResponse;
use crate::domain::auth::dto::auth_dto::*;
use std::sync::Arc;

pub async fn register(
    State(app_state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<RegisterDto>,
) -> (StatusCode, Json<ApiResponse<UserResponse>>) {
    let result = app_state.auth_service.register(payload).await;
    match result {
        Ok(user_response) => ApiResponse::success(user_response, "Registration successful"),
        Err(err_msg) => ApiResponse::error(&err_msg, StatusCode::BAD_REQUEST),
    }
}

pub async fn login(
    State(app_state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<LoginDto>,
) -> (StatusCode, Json<ApiResponse<UserResponse>>) {
    let result = app_state.auth_service.login(payload).await;
    match result {
        Ok(user_response) => ApiResponse::success(user_response, "Login successful"),
        Err(err_msg) => ApiResponse::error(&err_msg, StatusCode::UNAUTHORIZED),
    }
}
