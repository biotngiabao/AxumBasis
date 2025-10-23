use crate::domain::auth::{
    dto::auth_dto::{LoginDto, RegisterDto, UserResponse},
    infra::repo::AuthRepo,
};
use crate::entities::users;
use bcrypt::{DEFAULT_COST, verify};
use std::sync::Arc;

pub struct AuthService {
    pub auth_repo: AuthRepo,
}

impl AuthService {
    pub fn new(auth_repo: AuthRepo) -> Arc<Self> {
        Arc::new(Self { auth_repo })
    }
    pub async fn register(&self, payload: RegisterDto) -> Result<UserResponse, String> {
        let find_user = self.auth_repo.get_user_by_email(&payload.email).await;

        if let Ok(Some(_)) = find_user {
            return Err("Email already in use".to_string());
        }

        // hash password
        let hash_password = match bcrypt::hash(payload.password, DEFAULT_COST) {
            Ok(hash) => hash,
            Err(err) => {
                return Err(err.to_string());
            }
        };

        let new_user = users::ActiveModel {
            username: sea_orm::Set(payload.username.clone()),
            email: sea_orm::Set(payload.email.clone()),
            password: sea_orm::Set(hash_password),
            ..Default::default()
        };

        match self.auth_repo.create_user(new_user).await {
            Ok(user) => Ok(UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
            }),
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn login(&self, payload: LoginDto) -> Result<UserResponse, String> {
        let result = self.auth_repo.get_user_by_email(&payload.email).await;

        let user = match result {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Err("User not found".to_string());
            }
            Err(err) => {
                return Err(err.to_string());
            }
        };

        let is_valid = verify(payload.password, &user.password).unwrap_or(false);

        if !is_valid {
            return Err("Invalid password".to_string());
        }

        Ok(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
        })
    }
}
