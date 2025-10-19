use anyhow::{Context, Result};
use lib::config::CliConfig;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{client::ApiClient, LocationCommands};

#[derive(Debug, Serialize)]
struct CreateLocationRequest {
    name: String,
    timezone: String,
    latitude: Option<f32>,
    longitude: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct LocationResponse {
    id: Uuid,
    name: String,
    timezone: String,
    latitude: Option<f32>,
    longitude: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct LocationTimeResponse {
    location: LocationResponse,
    formatted_time: String,
}

pub async fn handle(command: LocationCommands, config: &CliConfig) -> Result<()> {
    let client = ApiClient::new(config);

    match command {
        LocationCommands::Add { name, tz, lat, lon } => {
            add_location(&client, name, tz, lat, lon).await?;
        }
        LocationCommands::List => {
            list_locations(&client).await?;
        }
        LocationCommands::Time { name } => {
            show_times(&client, name).await?;
        }
        LocationCommands::WatchAll => {
            watch_all_locations(config).await?;
        }
    }

    Ok(())
}

async fn add_location(
    client: &ApiClient,
    name: String,
    tz: Option<String>,
    lat: Option<f32>,
    lon: Option<f32>,
) -> Result<()> {
    let (timezone, latitude, longitude) = match (tz, lat, lon) {
        (Some(tz), lat, lon) => {
            // User provided timezone, use as-is
            (tz, lat, lon)
        }
        (None, _, _) => {
            // Auto-geocode the location
            println!("Looking up location data for '{}'...", name);

            let url = format!("/api/v1/geocoding/lookup?q={}", urlencoding::encode(&name));
            let response = client.get(&url).await?;

            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await?;
                anyhow::bail!("Failed to lookup location: {} - {}\nTry providing --tz, --lat, and --lon manually", status, body);
            }

            #[derive(Deserialize)]
            struct GeocodingResult {
                city_name: String,
                country: String,
                state: Option<String>,
                latitude: f32,
                longitude: f32,
                timezone: String,
            }

            let geo: GeocodingResult = response.json().await?;

            println!("Found: {} ({}{})",
                geo.city_name,
                geo.country,
                geo.state.map(|s| format!(", {}", s)).unwrap_or_default());

            (geo.timezone, Some(geo.latitude), Some(geo.longitude))
        }
    };

    let req = CreateLocationRequest {
        name: name.clone(),
        timezone: timezone.clone(),
        latitude,
        longitude,
    };

    let response = client.post("/api/v1/locations", &req).await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await?;
        anyhow::bail!("Failed to create location: {} - {}", status, body);
    }

    let location: LocationResponse = response.json().await?;

    println!("Location created!");
    println!("  Name: {}", location.name);
    println!("  ID: {}", location.id);
    println!("  Timezone: {}", location.timezone);
    if let (Some(lat), Some(lon)) = (location.latitude, location.longitude) {
        println!("  Coordinates: {}, {}", lat, lon);
    }

    Ok(())
}

async fn list_locations(client: &ApiClient) -> Result<()> {
    let response = client.get("/api/v1/locations").await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await?;
        anyhow::bail!("Failed to list locations: {} - {}", status, body);
    }

    let locations: Vec<LocationResponse> = response.json().await?;

    if locations.is_empty() {
        println!("No locations found.");
        return Ok(());
    }

    println!("Locations:");
    println!();
    for location in locations {
        print!("  {} - {} ({})", location.id, location.name, location.timezone);
        if let (Some(lat), Some(lon)) = (location.latitude, location.longitude) {
            println!(" - {}, {}", lat, lon);
        } else {
            println!();
        }
    }

    Ok(())
}

async fn show_times(client: &ApiClient, name: Option<String>) -> Result<()> {
    match name {
        Some(name) => {
            // Get time for specific location by name
            // First, get the location by name
            let locations_response = client.get("/api/v1/locations").await?;
            if !locations_response.status().is_success() {
                let status = locations_response.status();
                let body = locations_response.text().await?;
                anyhow::bail!("Failed to get locations: {} - {}", status, body);
            }

            let locations: Vec<LocationResponse> = locations_response.json().await?;
            let location = locations
                .iter()
                .find(|l| l.name == name)
                .context(format!("Location '{}' not found", name))?;

            // Get time for this location
            let url = format!("/api/v1/locations/{}/time", location.id);
            let response = client.get(&url).await?;

            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await?;
                anyhow::bail!("Failed to get time: {} - {}", status, body);
            }

            let time_response: LocationTimeResponse = response.json().await?;
            println!("{}: {}", time_response.location.name, time_response.formatted_time);
        }
        None => {
            // Get times for all locations
            let response = client.get("/api/v1/locations/times").await?;

            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await?;
                anyhow::bail!("Failed to get times: {} - {}", status, body);
            }

            let times: Vec<LocationTimeResponse> = response.json().await?;

            if times.is_empty() {
                println!("No locations found.");
                return Ok(());
            }

            println!("Current times:");
            println!();
            for time in times {
                println!("  {}: {}", time.location.name, time.formatted_time);
            }
        }
    }

    Ok(())
}

async fn watch_all_locations(config: &CliConfig) -> Result<()> {
    // Import the TUI module
    use super::super::tui::location_watch_all;

    // Run the TUI
    location_watch_all::run(config).await
}
