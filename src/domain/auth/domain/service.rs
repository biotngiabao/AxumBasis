use sea_orm::{ ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter };
use crate::domain::auth::dto::auth_dto::{ LoginDto, RegisterDto, UserResponse };
use bcrypt::{ verify, DEFAULT_COST };
use crate::entities::users;

#[derive(Clone)]
pub struct AuthService {
    pub db: DatabaseConnection,
}

impl AuthService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
    pub async fn register(&self, payload: RegisterDto) -> Result<UserResponse, String> {
        let find_user = users::Entity
            ::find()
            .filter(users::Column::Email.eq(payload.email.clone()))
            .one(&self.db).await;

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

        match new_user.insert(&self.db).await {
            Ok(user) =>
                Ok(UserResponse {
                    id: user.id,
                    username: user.username,
                    email: user.email,
                }),
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn login(&self, payload: LoginDto) -> Result<UserResponse, String> {
        let result = users::Entity
            ::find()
            .filter(users::Column::Email.eq(payload.email))
            .one(&self.db).await;

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
