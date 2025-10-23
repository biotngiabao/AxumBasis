use crate::dto::auth_dto::*;
use crate::extractor::ValidatedJson;
use crate::response::ApiResponse;
use axum::{Json, extract::Extension, http::StatusCode};
use bcrypt::{DEFAULT_COST, bcrypt, verify};
use sea_orm::DatabaseConnection;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use crate::entities::users;
// router (db) --> service (db) -> repo (db)
// router --> service -> repo (db)

pub async fn register(
    Extension(db): Extension<DatabaseConnection>,
    ValidatedJson(payload): ValidatedJson<RegisterDto>,
) -> (StatusCode, Json<ApiResponse<UserResponse>>) {
    let find_user = users::Entity::find()
        .filter(users::Column::Email.eq(payload.email.clone()))
        .one(&db)
        .await;

    if let Ok(Some(_)) = find_user {
        return ApiResponse::error("Email already in use", StatusCode::CONFLICT);
    }

    // hash password
    let hash_password = match bcrypt::hash(payload.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(err) => {
            return ApiResponse::error(&err.to_string(), StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let new_user = users::ActiveModel {
        username: sea_orm::Set(payload.username.clone()),
        email: sea_orm::Set(payload.email.clone()),
        password: sea_orm::Set(hash_password),
        ..Default::default()
    };

    return match new_user.insert(&db).await {
        Ok(user) => {
            let user_response = UserResponse {
                id: user.id as i32,
                username: user.username,
                email: user.email,
            };
            ApiResponse::success(user_response, "User registered successfully")
        }
        Err(err) => ApiResponse::error(
            &format!("Failed to register user: {}", err),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    };
}

pub async fn login(
    Extension(db): Extension<DatabaseConnection>,
    ValidatedJson(payload): ValidatedJson<LoginDto>,
) -> (StatusCode, Json<ApiResponse<UserResponse>>) {
    let result = users::Entity::find()
        .filter(users::Column::Email.eq(payload.email.clone()))
        .one(&db)
        .await;

    let find_user = match result {
        Ok(Some(user)) => user,
        Ok(None) => {
            return ApiResponse::error("User not found", StatusCode::NOT_FOUND);
        }
        Err(err) => {
            return ApiResponse::error(&err.to_string(), StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let is_valid = verify(payload.password, &find_user.password).unwrap_or(false);

    if !is_valid {
        return ApiResponse::error("Incorect password", StatusCode::UNAUTHORIZED);
    }

    return ApiResponse::success(
        UserResponse {
            id: find_user.id,
            username: find_user.username,
            email: find_user.email, // assuming you added it to UserResponse
        },
        "Login",
    );
}
