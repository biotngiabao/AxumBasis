use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct PingResponse {
    pub code: i32,
    pub message: String,
}

#[derive(Serialize)]
pub struct ServerInfoResponse {
    pub server_name: String,
    pub major_version: i32,
    pub minor_version: i32,
}

#[derive(Deserialize)]
pub struct PCARequest {
    pub file_path: String,
    pub stream: bool,
    pub n_components: i32,
}

#[derive(Serialize)]
pub struct PCAResponse {
    pub dtypes: String,
    pub array_bytes: Vec<u8>,
    pub shape: Vec<i32>,
}

