use axum::{ Json, extract::{ FromRequest }, http::StatusCode };
use serde::de::DeserializeOwned;
use validator::Validate;
use crate::dto::response::ApiResponse;

pub struct ValidatedJson<T>(pub T);
impl<B, T> FromRequest<B>
    for ValidatedJson<T>
    where T: DeserializeOwned + Validate + Send, B: Send + Sync
{
    type Rejection = (StatusCode, Json<ApiResponse<()>>);
    async fn from_request(req: axum::extract::Request, state: &B) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>
            ::from_request(req, state).await
            .map_err(|e| ApiResponse::error(&e.to_string(), StatusCode::BAD_REQUEST))?;

        value
            .validate()
            .map_err(|e|
                ApiResponse::<()>::error(
                    &format!("Validation failed: {e}"),
                    StatusCode::BAD_REQUEST
                )
            )?;

        Ok(ValidatedJson(value))
    }
}
