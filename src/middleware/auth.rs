use axum::{
    extract::Request,
    http::{ self, StatusCode },
    middleware::{ Next },
    response::Response,
    Router,
};

pub async fn auth_middleware(request: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = request.headers().get(http::header::AUTHORIZATION);
    match auth_header {
        Some(_) => { Ok(next.run(request).await) }
        _ => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }
}
