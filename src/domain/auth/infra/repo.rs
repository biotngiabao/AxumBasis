use crate::entities::users;
use sea_orm::{self, ActiveModelTrait, DbErr, EntityTrait, QueryFilter};
use sea_orm::{ColumnTrait, DatabaseConnection};

pub struct AuthRepo {
    pub db: DatabaseConnection,
}

impl AuthRepo {
    pub fn new(db: sea_orm::DatabaseConnection) -> Self {
        AuthRepo { db }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<users::Model>, DbErr> {
        let find_user = users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .one(&self.db)
            .await;
        return find_user;
    }

    pub async fn create_user(&self, new_user: users::ActiveModel) -> Result<users::Model, DbErr> {
        let user = new_user.insert(&self.db).await;
        return user;
    }
}
