use axum::{http::StatusCode, response::Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub message: String,
    pub success: bool,
    pub code: u16,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> (StatusCode, Json<Self>) {
        (
            StatusCode::OK,
            Json(ApiResponse {
                message: message.to_string(),
                success: true,
                code: 200,
                data: Some(data),
            }),
        )
    }

    pub fn error(message: &str, code: StatusCode) -> (StatusCode, Json<Self>) {
        (
            code.into(),
            Json(ApiResponse {
                message: message.to_string(),
                success: false,
                code: code.as_u16(),
                data: None,
            }),
        )
    }
}
