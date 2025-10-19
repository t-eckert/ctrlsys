use anyhow::{Context, Result};
use lib::config::CliConfig;
use serde::Deserialize;
use uuid::Uuid;

use crate::{client::ApiClient, WeatherCommands};

#[derive(Debug, Deserialize)]
struct LocationResponse {
    id: Uuid,
    name: String,
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    location_name: String,
    temperature_celsius: f32,
    temperature_fahrenheit: f32,
    feels_like_celsius: f32,
    humidity: u8,
    description: String,
    wind_speed_ms: f32,
    wind_speed_mph: f32,
}

pub async fn handle(command: WeatherCommands, config: &CliConfig) -> Result<()> {
    let client = ApiClient::new(config);

    match command {
        WeatherCommands::Get { name } => {
            get_weather(&client, name).await?;
        }
        WeatherCommands::WatchAll => {
            watch_all_weather(config).await?;
        }
    }

    Ok(())
}

async fn get_weather(client: &ApiClient, name: Option<String>) -> Result<()> {
    match name {
        Some(name) => {
            // Get weather for specific location by name
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

            // Get weather for this location
            let url = format!("/api/v1/weather/locations/{}", location.id);
            let response = client.get(&url).await?;

            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await?;
                anyhow::bail!("Failed to get weather: {} - {}", status, body);
            }

            let weather: WeatherResponse = response.json().await?;
            print_weather(&weather);
        }
        None => {
            // Get weather for all locations
            let response = client.get("/api/v1/weather/locations").await?;

            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await?;
                anyhow::bail!("Failed to get weather: {} - {}", status, body);
            }

            let weather_list: Vec<WeatherResponse> = response.json().await?;

            if weather_list.is_empty() {
                println!("No locations with weather data found.");
                println!("Make sure locations have latitude and longitude set.");
                return Ok(());
            }

            println!("Current weather:");
            println!();
            for weather in weather_list {
                print_weather(&weather);
                println!();
            }
        }
    }

    Ok(())
}

fn print_weather(weather: &WeatherResponse) {
    println!("{}:", weather.location_name);
    println!("  Temperature: {:.1}C / {:.1}F",
        weather.temperature_celsius,
        weather.temperature_fahrenheit);
    println!("  Feels like: {:.1}C", weather.feels_like_celsius);
    println!("  Conditions: {}", weather.description);
    println!("  Humidity: {}%", weather.humidity);
    println!("  Wind: {:.1} m/s ({:.1} mph)",
        weather.wind_speed_ms,
        weather.wind_speed_mph);
}

async fn watch_all_weather(config: &CliConfig) -> Result<()> {
    // Import the TUI module
    use super::super::tui::weather_watch_all;

    // Run the TUI
    weather_watch_all::run(config).await
}
