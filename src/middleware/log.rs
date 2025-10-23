use axum::{
    Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
};

pub async fn log_middleware(request: Request, next: Next) -> Response {
    println!("Before::{}", &request.uri());
    let response = next.run(request).await;

    return response;
}
