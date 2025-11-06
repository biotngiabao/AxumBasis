use axum::extract::State;
use axum::{Json, http::StatusCode};

use crate::common::bootstap::AppState;
use crate::common::response::ApiResponse;
use crate::domain::herax::dto::ping_dto::*;

use std::sync::Arc;

pub async fn ping(
    State(app_state): State<Arc<AppState>>,
) -> (StatusCode, Json<ApiResponse<PingResponse>>) {
    match app_state.grpc_client.ping().await {
        Ok(message) => ApiResponse::success(
            PingResponse {
                code: message.code,
                message: message.message,
            },
            "Ping successfully",
        ),
        Err(err) => ApiResponse::error(
            &format!("Failed to ping: {}", err),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}

pub async fn get_server_info(
    State(app_state): State<Arc<AppState>>,
) -> (StatusCode, Json<ApiResponse<ServerInfoResponse>>) {
    match app_state.grpc_client.get_server_info().await {
        Ok(data) => {
            return ApiResponse::success(
                ServerInfoResponse {
                    server_name: data.server_name,
                    major_version: data.major_version,
                    minor_version: data.minor_version,
                },
                "Get info successfully",
            );
        }
        Err(err) => ApiResponse::error(
            &format!("Failed to ping: {}", err),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}
