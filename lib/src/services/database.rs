use anyhow::{Context, Result};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::database::{CreateDatabaseRequest, ManagedDatabase};

pub struct DatabaseService;

impl DatabaseService {
    /// Create a new database and track it
    pub async fn create(pool: &PgPool, req: CreateDatabaseRequest) -> Result<ManagedDatabase> {
        // Validate database name (only alphanumeric and underscores, must start with letter)
        Self::validate_db_name(&req.db_name)?;

        let id = Uuid::new_v4();
        let now = chrono::Utc::now();

        // Create the actual database on the Postgres server
        // IMPORTANT: We cannot use parameterized queries for CREATE DATABASE
        // So we must carefully validate the name first
        let create_db_query = format!("CREATE DATABASE \"{}\"", req.db_name);
        sqlx::query(&create_db_query)
            .execute(pool)
            .await
            .context("Failed to create database")?;

        // Track the database in our managed_databases table
        let managed_db = sqlx::query_as::<_, ManagedDatabase>(
            r#"
            INSERT INTO managed_databases (id, db_name, created_at, owner, notes)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&req.db_name)
        .bind(now)
        .bind(&req.owner)
        .bind(&req.notes)
        .fetch_one(pool)
        .await?;

        Ok(managed_db)
    }

    /// List all managed databases
    pub async fn list(pool: &PgPool) -> Result<Vec<ManagedDatabase>> {
        let databases = sqlx::query_as::<_, ManagedDatabase>(
            r#"
            SELECT * FROM managed_databases
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(databases)
    }

    /// Get a specific managed database by name
    pub async fn get_by_name(pool: &PgPool, db_name: &str) -> Result<Option<ManagedDatabase>> {
        let database = sqlx::query_as::<_, ManagedDatabase>(
            r#"
            SELECT * FROM managed_databases
            WHERE db_name = $1
            "#,
        )
        .bind(db_name)
        .fetch_optional(pool)
        .await?;

        Ok(database)
    }

    /// Get a specific managed database by ID
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<ManagedDatabase>> {
        let database = sqlx::query_as::<_, ManagedDatabase>(
            r#"
            SELECT * FROM managed_databases
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(database)
    }

    /// Drop a database and remove it from tracking
    pub async fn drop(pool: &PgPool, db_name: &str) -> Result<ManagedDatabase> {
        // Safety check: prevent dropping certain important databases
        let protected_databases = ["postgres", "template0", "template1", "ctrlsys"];
        if protected_databases.contains(&db_name) {
            anyhow::bail!("Cannot drop protected database: {}", db_name);
        }

        // Verify the database is tracked
        let managed_db = Self::get_by_name(pool, db_name)
            .await?
            .context("Database not found in managed databases")?;

        // Validate database name (extra safety)
        Self::validate_db_name(db_name)?;

        // Terminate existing connections to the database
        let terminate_query = format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}'",
            db_name
        );
        sqlx::query(&terminate_query).execute(pool).await?;

        // Drop the actual database
        let drop_db_query = format!("DROP DATABASE \"{}\"", db_name);
        sqlx::query(&drop_db_query)
            .execute(pool)
            .await
            .context("Failed to drop database")?;

        // Remove from tracked databases
        sqlx::query(
            r#"
            DELETE FROM managed_databases
            WHERE db_name = $1
            "#,
        )
        .bind(db_name)
        .execute(pool)
        .await?;

        Ok(managed_db)
    }

    /// Check if a database exists on the Postgres server
    pub async fn exists(pool: &PgPool, db_name: &str) -> Result<bool> {
        let result: Option<(bool,)> = sqlx::query_as(
            r#"
            SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)
            "#,
        )
        .bind(db_name)
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|(exists,)| exists).unwrap_or(false))
    }

    /// Validate database name to prevent SQL injection
    fn validate_db_name(name: &str) -> Result<()> {
        if name.is_empty() {
            anyhow::bail!("Database name cannot be empty");
        }

        if name.len() > 63 {
            anyhow::bail!("Database name cannot exceed 63 characters");
        }

        // Must start with a letter or underscore
        let first_char = name.chars().next().unwrap();
        if !first_char.is_ascii_alphabetic() && first_char != '_' {
            anyhow::bail!("Database name must start with a letter or underscore");
        }

        // Can only contain alphanumeric characters and underscores
        if !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            anyhow::bail!(
                "Database name can only contain letters, numbers, and underscores"
            );
        }

        Ok(())
    }
}
