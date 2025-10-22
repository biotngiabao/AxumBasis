use axum::{ Json, extract::{ FromRequest, Extension }, http::StatusCode };
use bcrypt::{ bcrypt, verify, DEFAULT_COST };
use sea_orm::{ ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter };
use serde::de::DeserializeOwned;
use serde::{ Deserialize, Serialize };
use validator::Validate;
use crate::dto::response::ApiResponse;
use sea_orm::DatabaseConnection;

use crate::entities::users;

#[derive(Clone, Deserialize, Validate)]
pub struct RegisterDto {
    #[validate(length(min = 3, message = "username must be at least 3 characters"))]
    pub username: String,
    #[validate(
        length(min = 3, message = "email must be at least 3 characters"),
        email(message = "email must be a valid email address")
    )]
    pub email: String,
    #[validate(length(min = 6, message = "password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Clone, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Clone, Deserialize, Validate)]
pub struct LoginDto {
    #[validate(length(min = 3, message = "email must be at least 3 characters"))]
    pub email: String,
    #[validate(length(min = 6, message = "password must be at least 6 characters"))]
    pub password: String,
}
