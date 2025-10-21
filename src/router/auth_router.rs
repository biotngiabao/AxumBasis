use axum::{Json, extract::FromRequest, http::StatusCode};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Deserialize, Validate)]
pub struct RegisterDto {
    #[validate(length(min = 3, message = "username must be at least 3 characters"))]
    username: String,
    #[validate(length(min = 6, message = "password must be at least 6 characters"))]
    password: String,
}

#[derive(Clone, Serialize)]
pub struct UserResponse {
    id: u32,
    username: String,
}

pub struct ValidatedJson<T>(pub T);
impl<B, T> FromRequest<B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + Send,
    B: Send + Sync,
{
    type Rejection = (StatusCode, String);
    async fn from_request(req: axum::extract::Request, state: &B) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        value.validate().map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("Validation failed: {:?}", e.to_string()),
            )
        })?;

        Ok(ValidatedJson(value))
    }
}

pub async fn register(
    Json(payload): Json<RegisterDto>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    if let Err(errors) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Validation failed: {:?}", errors.to_string()),
        ));
    }

    let res = UserResponse {
        id: 1,
        username: payload.username,
    };

    return Ok(Json(res));
}

pub async fn registerv2(
    ValidatedJson(payload): ValidatedJson<RegisterDto>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let res = UserResponse {
        id: 1,
        username: payload.username,
    };

    return Ok(Json(res));
}
