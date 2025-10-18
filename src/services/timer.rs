use anyhow::Result;
use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::timer::{CreateTimerRequest, Timer, TimerResponse, TimerStatus};

pub struct TimerService;

impl TimerService {
    /// Create a new timer
    pub async fn create(pool: &PgPool, req: CreateTimerRequest) -> Result<Timer> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let timer = sqlx::query_as::<_, Timer>(
            r#"
            INSERT INTO timers (id, name, duration_seconds, created_at, status)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&req.name)
        .bind(req.duration_seconds)
        .bind(now)
        .bind(TimerStatus::Pending)
        .fetch_one(pool)
        .await?;

        Ok(timer)
    }

    /// Get a timer by ID
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Timer>> {
        let timer = sqlx::query_as::<_, Timer>(
            r#"
            SELECT * FROM timers WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(timer)
    }

    /// List all timers (excludes completed timers older than 24 hours)
    pub async fn list(pool: &PgPool) -> Result<Vec<Timer>> {
        let cutoff = Utc::now() - Duration::hours(24);

        let timers = sqlx::query_as::<_, Timer>(
            r#"
            SELECT * FROM timers
            WHERE
                status != $1
                OR (status = $1 AND created_at >= $2)
            ORDER BY
                CASE
                    WHEN status = 'running' THEN 1
                    WHEN status = 'pending' THEN 2
                    WHEN status = 'completed' THEN 3
                    WHEN status = 'cancelled' THEN 4
                END,
                created_at DESC
            "#,
        )
        .bind(TimerStatus::Completed)
        .bind(cutoff)
        .fetch_all(pool)
        .await?;

        Ok(timers)
    }

    /// Start a timer
    pub async fn start(pool: &PgPool, id: Uuid) -> Result<Option<Timer>> {
        let now = Utc::now();

        // Get the timer first to calculate expiration
        let timer = Self::get_by_id(pool, id).await?;
        let Some(timer) = timer else {
            return Ok(None);
        };

        if timer.status != TimerStatus::Pending {
            return Ok(Some(timer));
        }

        let expires_at = now + Duration::seconds(timer.duration_seconds as i64);

        let timer = sqlx::query_as::<_, Timer>(
            r#"
            UPDATE timers
            SET status = $1, started_at = $2, expires_at = $3
            WHERE id = $4
            RETURNING *
            "#,
        )
        .bind(TimerStatus::Running)
        .bind(now)
        .bind(expires_at)
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(Some(timer))
    }

    /// Cancel a timer
    pub async fn cancel(pool: &PgPool, id: Uuid) -> Result<Option<Timer>> {
        let timer = sqlx::query_as::<_, Timer>(
            r#"
            UPDATE timers
            SET status = $1
            WHERE id = $2 AND status != $3
            RETURNING *
            "#,
        )
        .bind(TimerStatus::Cancelled)
        .bind(id)
        .bind(TimerStatus::Completed)
        .fetch_optional(pool)
        .await?;

        Ok(timer)
    }

    /// Mark expired timers as completed
    pub async fn complete_expired_timers(pool: &PgPool) -> Result<Vec<Timer>> {
        let now = Utc::now();

        let timers = sqlx::query_as::<_, Timer>(
            r#"
            UPDATE timers
            SET status = $1
            WHERE status = $2 AND expires_at <= $3
            RETURNING *
            "#,
        )
        .bind(TimerStatus::Completed)
        .bind(TimerStatus::Running)
        .bind(now)
        .fetch_all(pool)
        .await?;

        Ok(timers)
    }

    /// Get all running timers
    pub async fn get_running(pool: &PgPool) -> Result<Vec<Timer>> {
        let timers = sqlx::query_as::<_, Timer>(
            r#"
            SELECT * FROM timers
            WHERE status = $1
            ORDER BY expires_at ASC
            "#,
        )
        .bind(TimerStatus::Running)
        .fetch_all(pool)
        .await?;

        Ok(timers)
    }
}

/// Convert a Timer to a TimerResponse
pub fn to_response(timer: Timer) -> TimerResponse {
    TimerResponse::from(timer)
}
