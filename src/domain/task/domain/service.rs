use crate::domain::task::dto::task_dto::*;
use crate::domain::task::infra::repo::TaskRepo;
use std::sync::Arc;

pub struct TaskService {
    pub task_repo: TaskRepo,
}

impl TaskService {
    pub fn new(task_repo: TaskRepo) -> Arc<Self> {
        return Arc::new(TaskService { task_repo });
    }
    pub async fn create_task(&self, payload: TaskCreated) -> anyhow::Result<TaskResponse> {
        let task = self.task_repo.create_task(payload).await?;
        Ok(TaskResponse {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            user_id: task.user_id,
            is_default: task.is_default,
        })
    }

    pub async fn get_task_by_id(&self, id: i32) -> anyhow::Result<TaskResponse> {
        match self.task_repo.get_task_by_id(id).await {
            Ok(Some(task)) => {
                let task_response = TaskResponse {
                    id: task.id,
                    title: task.title,
                    priority: task.priority,
                    description: task.description,
                    user_id: task.user_id,
                    is_default: task.is_default,
                };
                Ok(task_response)
            }
            Ok(None) => Err(anyhow::anyhow!("Task not found for id: {}", id)),
            Err(err) => Err(err.into()),
        }
    }

    pub async fn get_all_tasks(&self) -> anyhow::Result<Vec<TaskResponse>> {
        match self.task_repo.get_all_tasks().await {
            Ok(tasks) => {
                let task_responses: Vec<TaskResponse> = tasks
                    .into_iter()
                    .map(|task| TaskResponse {
                        id: task.id,
                        title: task.title,
                        priority: task.priority,
                        description: task.description,
                        user_id: task.user_id,
                        is_default: task.is_default,
                    })
                    .collect();
                Ok(task_responses)
            }
            Err(err) => Err(err.into()),
        }
    }
}
