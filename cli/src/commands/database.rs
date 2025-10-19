use anyhow::Result;
use chrono::{DateTime, Utc};
use lib::config::CliConfig;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use uuid::Uuid;

use crate::{client::ApiClient, DatabaseCommands};

#[derive(Debug, Serialize)]
struct CreateDatabaseRequest {
    db_name: String,
    owner: Option<String>,
    notes: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ManagedDatabase {
    id: Uuid,
    db_name: String,
    created_at: DateTime<Utc>,
    owner: Option<String>,
    notes: Option<String>,
}

pub async fn handle(command: DatabaseCommands, config: &CliConfig) -> Result<()> {
    let client = ApiClient::new(config);

    match command {
        DatabaseCommands::Create { name } => {
            create_database(&client, name).await?;
        }
        DatabaseCommands::List => {
            list_databases(&client).await?;
        }
        DatabaseCommands::Drop { name } => {
            drop_database(&client, name).await?;
        }
    }

    Ok(())
}

async fn create_database(client: &ApiClient, name: String) -> Result<()> {
    println!("Creating database '{}'...", name);

    let req = CreateDatabaseRequest {
        db_name: name.clone(),
        owner: None,
        notes: None,
    };

    let response = client.post("/api/v1/databases", &req).await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await?;
        anyhow::bail!("Failed to create database: {} - {}", status, body);
    }

    let database: ManagedDatabase = response.json().await?;

    println!("Database created successfully!");
    println!("  Name: {}", database.db_name);
    println!("  ID: {}", database.id);
    println!("  Created: {}", database.created_at.format("%Y-%m-%d %H:%M:%S UTC"));

    Ok(())
}

async fn list_databases(client: &ApiClient) -> Result<()> {
    let response = client.get("/api/v1/databases").await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await?;
        anyhow::bail!("Failed to list databases: {} - {}", status, body);
    }

    let databases: Vec<ManagedDatabase> = response.json().await?;

    if databases.is_empty() {
        println!("No managed databases found.");
        return Ok(());
    }

    println!("Managed Databases:");
    println!();
    for db in databases {
        println!("  {} - {}", db.db_name, db.id);
        println!("    Created: {}", db.created_at.format("%Y-%m-%d %H:%M:%S UTC"));
        if let Some(owner) = &db.owner {
            println!("    Owner: {}", owner);
        }
        if let Some(notes) = &db.notes {
            println!("    Notes: {}", notes);
        }
        println!();
    }

    Ok(())
}

async fn drop_database(client: &ApiClient, name: String) -> Result<()> {
    // Confirmation prompt
    print!("WARNING: This will permanently delete database '{}'. Continue? (yes/no): ", name);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    if input != "yes" && input != "y" {
        println!("Operation cancelled.");
        return Ok(());
    }

    println!("Dropping database '{}'...", name);

    let url = format!("/api/v1/databases/{}", name);
    let response = client.delete(&url).await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await?;
        anyhow::bail!("Failed to drop database: {} - {}", status, body);
    }

    let database: ManagedDatabase = response.json().await?;

    println!("Database '{}' dropped successfully!", database.db_name);

    Ok(())
}
