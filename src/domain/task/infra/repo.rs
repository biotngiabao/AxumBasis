use crate::domain::task::dto::task_dto::TaskCreated;
use crate::entities::tasks;
use sea_orm::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};

pub struct TaskRepo {
    pub db: DatabaseConnection,
}

impl TaskRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        TaskRepo { db }
    }

    pub async fn get_task_by_id(&self, id: i32) -> Result<Option<tasks::Model>, sea_orm::DbErr> {
        let task = tasks::Entity::find_by_id(id).one(&self.db).await;
        return task;
    }

    pub async fn get_all_tasks(&self) -> Result<Vec<tasks::Model>, sea_orm::DbErr> {
        let task_list = tasks::Entity::find().all(&self.db).await;
        return task_list;
    }

    pub async fn create_task(&self, payload: TaskCreated) -> Result<tasks::Model, sea_orm::DbErr> {
        let new_task = tasks::ActiveModel {
            priority: Set(payload.priority),
            title: Set(payload.title),
            description: Set(payload.description),
            user_id: Set(payload.user_id),
            is_default: Set(payload.is_default),
            ..Default::default()
        };

        return new_task.insert(&self.db).await;
    }
}
