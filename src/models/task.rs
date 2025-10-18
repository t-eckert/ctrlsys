use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub total_time_seconds: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum TaskStatus {
    #[sqlx(rename = "todo")]
    Todo,
    #[sqlx(rename = "in_progress")]
    InProgress,
    #[sqlx(rename = "completed")]
    Completed,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "todo"),
            TaskStatus::InProgress => write!(f, "in_progress"),
            TaskStatus::Completed => write!(f, "completed"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub status: Option<TaskStatus>,
    pub description: Option<String>,
}
