use axum::body;
use axum::extract::State;
use axum::{Json, http::StatusCode};

use crate::app;
use crate::common::bootstap::AppState;
use crate::common::proto;
// use crate::common::proto::*;
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

pub async fn pca(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<PCARequest>,
) -> (StatusCode, Json<ApiResponse<PCAResponse>>) {
    match app_state
        .grpc_client
        .pca(body.file_path, body.stream, body.n_components)
        .await
    {
        Ok(value) => match value {
            proto::dimred_pca_response::Value::Data(data) => {
                return ApiResponse::success(
                    PCAResponse {
                        dtypes: data.dtype,
                        array_bytes: data.arr_bytes,
                        shape: data.shape,
                    },
                    "PCA successfully",
                );
            }
            (_) => {
                return ApiResponse::error("Fail", StatusCode::CONFLICT);
            }
        },
        Err(err) => ApiResponse::error(
            &format!("Failed to pca: {}", err),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}
