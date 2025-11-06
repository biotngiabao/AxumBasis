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
