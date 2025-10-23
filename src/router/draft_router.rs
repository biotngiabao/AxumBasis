use axum::{ http::header, Extension, Json };
use serde::{ Deserialize, Serialize };

use crate::router::SharedData;

pub async fn helloworld() -> String {
    return String::from("Hello world");
}

pub async fn get_body(body: String) -> String {
    return body;
}
#[derive(Deserialize, Serialize, Debug)]
pub struct HelloPayload {
    message: String,
}

pub async fn get_body_json(Json(payload): Json<HelloPayload>) -> Json<HelloPayload> {
    dbg!(&payload);
    return Json(payload);
}

pub async fn get_header(headers: header::HeaderMap) -> String {
    dbg!(&headers);
    let postman_token = headers.get("postman-token").unwrap();
    return postman_token.to_str().unwrap().to_owned();
}

pub async fn get_shared_data(Extension(data): Extension<SharedData>) -> String {
    println!("Message{}", data.message);
    return data.message;
}
