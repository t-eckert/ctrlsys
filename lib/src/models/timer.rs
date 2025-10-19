use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Timer {
    pub id: Uuid,
    pub name: String,
    pub duration_seconds: i32,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: TimerStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
#[serde(rename_all = "lowercase")]
pub enum TimerStatus {
    #[sqlx(rename = "pending")]
    Pending,
    #[sqlx(rename = "running")]
    Running,
    #[sqlx(rename = "completed")]
    Completed,
    #[sqlx(rename = "cancelled")]
    Cancelled,
}

impl std::fmt::Display for TimerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimerStatus::Pending => write!(f, "pending"),
            TimerStatus::Running => write!(f, "running"),
            TimerStatus::Completed => write!(f, "completed"),
            TimerStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTimerRequest {
    pub name: String,
    pub duration_seconds: i32,
}

#[derive(Debug, Serialize)]
pub struct TimerResponse {
    pub id: Uuid,
    pub name: String,
    pub duration_seconds: i32,
    pub status: TimerStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub remaining_seconds: Option<i32>,
}

impl From<Timer> for TimerResponse {
    fn from(timer: Timer) -> Self {
        let remaining_seconds = timer.expires_at.map(|expires| {
            let now = Utc::now();
            let remaining = (expires - now).num_seconds();
            remaining.max(0) as i32
        });

        TimerResponse {
            id: timer.id,
            name: timer.name,
            duration_seconds: timer.duration_seconds,
            status: timer.status,
            created_at: timer.created_at,
            started_at: timer.started_at,
            expires_at: timer.expires_at,
            remaining_seconds,
        }
    }
}
