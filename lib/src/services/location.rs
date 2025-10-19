use anyhow::Result;
use chrono::Utc;
use chrono_tz::Tz;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::location::{CreateLocationRequest, Location, LocationTimeResponse};

pub struct LocationService;

impl LocationService {
    /// Create a new location
    pub async fn create(pool: &PgPool, req: CreateLocationRequest) -> Result<Location> {
        // Validate timezone string
        let _: Tz = req.timezone.parse()?;

        let id = Uuid::new_v4();
        let now = Utc::now();

        let location = sqlx::query_as::<_, Location>(
            r#"
            INSERT INTO locations (id, name, timezone, latitude, longitude, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&req.name)
        .bind(&req.timezone)
        .bind(req.latitude)
        .bind(req.longitude)
        .bind(now)
        .fetch_one(pool)
        .await?;

        Ok(location)
    }

    /// Get a location by ID
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Location>> {
        let location = sqlx::query_as::<_, Location>(
            r#"
            SELECT * FROM locations WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(location)
    }

    /// Get a location by name
    pub async fn get_by_name(pool: &PgPool, name: &str) -> Result<Option<Location>> {
        let location = sqlx::query_as::<_, Location>(
            r#"
            SELECT * FROM locations WHERE name = $1
            "#,
        )
        .bind(name)
        .fetch_optional(pool)
        .await?;

        Ok(location)
    }

    /// List all locations
    pub async fn list(pool: &PgPool) -> Result<Vec<Location>> {
        let locations = sqlx::query_as::<_, Location>(
            r#"
            SELECT * FROM locations
            ORDER BY name ASC
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(locations)
    }

    /// Delete a location
    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Option<Location>> {
        let location = sqlx::query_as::<_, Location>(
            r#"
            DELETE FROM locations
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(location)
    }

    /// Get current time at a location
    pub async fn get_time(pool: &PgPool, id: Uuid) -> Result<Option<LocationTimeResponse>> {
        let location = Self::get_by_id(pool, id).await?;

        let Some(location) = location else {
            return Ok(None);
        };

        let timezone: Tz = location.timezone.parse()?;
        let now_utc = Utc::now();
        let now_local = now_utc.with_timezone(&timezone);

        let formatted_time = now_local.format("%Y-%m-%d %H:%M:%S %Z").to_string();

        Ok(Some(LocationTimeResponse {
            location,
            current_time: now_utc,
            formatted_time,
        }))
    }

    /// Get times for all locations
    pub async fn list_times(pool: &PgPool) -> Result<Vec<LocationTimeResponse>> {
        let locations = Self::list(pool).await?;
        let now_utc = Utc::now();

        let mut responses = Vec::new();

        for location in locations {
            let timezone: Tz = location.timezone.parse()?;
            let now_local = now_utc.with_timezone(&timezone);
            let formatted_time = now_local.format("%Y-%m-%d %H:%M:%S %Z").to_string();

            responses.push(LocationTimeResponse {
                location,
                current_time: now_utc,
                formatted_time,
            });
        }

        Ok(responses)
    }
}
