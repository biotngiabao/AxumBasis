// use axum::{ Json, extract::{ FromRequest, Extension }, http::StatusCode };
// use bcrypt::{ bcrypt, verify, DEFAULT_COST };
// use sea_orm::{ ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter };
// use serde::de::DeserializeOwned;
// use serde::{ Deserialize, Serialize };
// use validator::Validate;
// use crate::dto::response::ApiResponse;
// use sea_orm::DatabaseConnection;

// use crate::entities::users;

// #[derive(Clone, Deserialize, Validate)]
// pub struct RegisterDto {
//     #[validate(length(min = 3, message = "username must be at least 3 characters"))]
//     username: String,
//     #[validate(
//         length(min = 3, message = "email must be at least 3 characters"),
//         email(message = "email must be a valid email address")
//     )]
//     email: String,
//     #[validate(length(min = 6, message = "password must be at least 6 characters"))]
//     password: String,
// }

// #[derive(Clone, Serialize)]
// pub struct UserResponse {
//     id: i32,
//     username: String,
//     email: String,
// }

// pub struct ValidatedJson<T>(pub T);
// impl<B, T> FromRequest<B>
//     for ValidatedJson<T>
//     where T: DeserializeOwned + Validate + Send, B: Send + Sync
// {
//     type Rejection = (StatusCode, Json<ApiResponse<()>>);
//     async fn from_request(req: axum::extract::Request, state: &B) -> Result<Self, Self::Rejection> {
//         let Json(value) = Json::<T>
//             ::from_request(req, state).await
//             .map_err(|e| ApiResponse::error(&e.to_string(), StatusCode::BAD_REQUEST))?;

//         value
//             .validate()
//             .map_err(|e|
//                 ApiResponse::<()>::error(
//                     &format!("Validation failed: {e}"),
//                     StatusCode::BAD_REQUEST
//                 )
//             )?;

//         Ok(ValidatedJson(value))
//     }
// }

// pub async fn register(
//     Extension(db): Extension<DatabaseConnection>,
//     ValidatedJson(payload): ValidatedJson<RegisterDto>
// ) -> (StatusCode, Json<ApiResponse<UserResponse>>) {
//     let find_user = users::Entity
//         ::find()
//         .filter(users::Column::Email.eq(payload.email.clone()))
//         .one(&db).await;

//     if let Ok(Some(_)) = find_user {
//         return ApiResponse::error("Email already in use", StatusCode::CONFLICT);
//     }

//     // hash password
//     let hash_password = match bcrypt::hash(payload.password, DEFAULT_COST) {
//         Ok(hash) => hash,
//         Err(err) => {
//             return ApiResponse::error(&err.to_string(), StatusCode::INTERNAL_SERVER_ERROR);
//         }
//     };

//     let new_user = users::ActiveModel {
//         username: sea_orm::Set(payload.username.clone()),
//         email: sea_orm::Set(payload.email.clone()),
//         password: sea_orm::Set(hash_password),
//         ..Default::default()
//     };

//     return match new_user.insert(&db).await {
//         Ok(user) => {
//             let user_response = UserResponse {
//                 id: user.id as i32,
//                 username: user.username,
//                 email: user.email,
//             };
//             ApiResponse::success(user_response, "User registered successfully")
//         }
//         Err(err) =>
//             ApiResponse::error(
//                 &format!("Failed to register user: {}", err),
//                 StatusCode::INTERNAL_SERVER_ERROR
//             ),
//     };
// }

// #[derive(Clone, Deserialize, Validate)]
// pub struct LoginDto {
//     #[validate(length(min = 3, message = "email must be at least 3 characters"))]
//     pub email: String,
//     #[validate(length(min = 6, message = "password must be at least 6 characters"))]
//     pub password: String,
// }

// pub async fn login(
//     Extension(db): Extension<DatabaseConnection>,
//     ValidatedJson(payload): ValidatedJson<LoginDto>
// ) -> (StatusCode, Json<ApiResponse<UserResponse>>) {
//     let result = users::Entity
//         ::find()
//         .filter(users::Column::Email.eq(payload.email.clone()))
//         .one(&db).await;

//     let find_user = match result {
//         Ok(Some(user)) => user,
//         Ok(None) => {
//             return ApiResponse::error("User not found", StatusCode::NOT_FOUND);
//         }
//         Err(err) => {
//             return ApiResponse::error(&err.to_string(), StatusCode::INTERNAL_SERVER_ERROR);
//         }
//     };

//     let is_valid = verify(payload.password, &find_user.password).unwrap_or(false);

//     if !is_valid {
//         return ApiResponse::error("Incorect password", StatusCode::UNAUTHORIZED);
//     }

//     return ApiResponse::success(
//         UserResponse {
//             id: find_user.id,
//             username: find_user.username,
//             email: find_user.email, // assuming you added it to UserResponse
//         },
//         "Login"
//     );
// }
