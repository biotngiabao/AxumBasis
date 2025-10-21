use axum::{ extract::Request, middleware::{ self, Next }, response::Response, Router };

pub async fn log_middleware(request: Request, next: Next) -> Response {
    println!("Before::{}", &request.uri());
    let response = next.run(request).await;
    println!("After::");

    return response;
}
