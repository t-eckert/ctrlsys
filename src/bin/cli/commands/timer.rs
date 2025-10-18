use anyhow::{Context, Result};
use ctrlsys::config::CliConfig;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{client::ApiClient, TimerCommands};

#[derive(Debug, Serialize)]
struct CreateTimerRequest {
    name: String,
    duration_seconds: i32,
}

#[derive(Debug, Deserialize)]
struct TimerResponse {
    id: Uuid,
    name: String,
    duration_seconds: i32,
    status: String,
    remaining_seconds: Option<i32>,
}

pub async fn handle(command: TimerCommands, config: &CliConfig) -> Result<()> {
    let client = ApiClient::new(config);

    match command {
        TimerCommands::Create { name, duration } => {
            create_timer(&client, name, duration).await?;
        }
        TimerCommands::List => {
            list_timers(&client).await?;
        }
        TimerCommands::Watch { id } => {
            let timer_id = Uuid::parse_str(&id)
                .context("Invalid timer ID format")?;
            watch_timer(config, timer_id).await?;
        }
    }

    Ok(())
}

async fn create_timer(client: &ApiClient, name: String, duration: i32) -> Result<()> {
    let req = CreateTimerRequest {
        name: name.clone(),
        duration_seconds: duration,
    };

    let response = client.post("/api/v1/timers", &req).await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await?;
        anyhow::bail!("Failed to create timer: {} - {}", status, body);
    }

    let timer: TimerResponse = response.json().await?;

    println!("Timer created and started!");
    println!("  Name: {}", timer.name);
    println!("  ID: {}", timer.id);
    println!("  Duration: {} seconds", timer.duration_seconds);
    println!("\nWatch it with: cs timer watch {}", timer.id);

    Ok(())
}

async fn list_timers(client: &ApiClient) -> Result<()> {
    let response = client.get("/api/v1/timers").await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await?;
        anyhow::bail!("Failed to list timers: {} - {}", status, body);
    }

    let timers: Vec<TimerResponse> = response.json().await?;

    if timers.is_empty() {
        println!("No timers found.");
        return Ok(());
    }

    println!("Timers:");
    println!();
    for timer in timers {
        println!("  {} - {} ({})", timer.id, timer.name, timer.status);
        if let Some(remaining) = timer.remaining_seconds {
            println!("    Remaining: {} seconds", remaining);
        }
    }

    Ok(())
}

async fn watch_timer(config: &CliConfig, timer_id: Uuid) -> Result<()> {
    // Import the TUI module
    use super::super::tui::timer_watch;

    // Run the TUI
    timer_watch::run(config, timer_id).await
}
