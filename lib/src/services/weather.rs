use anyhow::{Context, Result};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::location::Location;
use crate::models::weather::{OpenWeatherResponse, WeatherResponse};
use crate::services::location::LocationService;

pub struct WeatherService;

impl WeatherService {
    /// Get weather for a specific location
    pub async fn get_for_location(
        pool: &PgPool,
        location_id: Uuid,
        api_key: &str,
    ) -> Result<WeatherResponse> {
        let location = LocationService::get_by_id(pool, location_id)
            .await?
            .context("Location not found")?;

        Self::fetch_weather(&location, api_key).await
    }

    /// Get weather for all locations
    pub async fn get_for_all_locations(
        pool: &PgPool,
        api_key: &str,
    ) -> Result<Vec<WeatherResponse>> {
        let locations = LocationService::list(pool).await?;
        let mut weather_responses = Vec::new();

        for location in locations {
            if location.latitude.is_some() && location.longitude.is_some() {
                match Self::fetch_weather(&location, api_key).await {
                    Ok(weather) => weather_responses.push(weather),
                    Err(e) => {
                        tracing::warn!(
                            "Failed to fetch weather for location {}: {}",
                            location.name,
                            e
                        );
                    }
                }
            }
        }

        Ok(weather_responses)
    }

    /// Fetch weather from OpenWeatherMap API
    async fn fetch_weather(location: &Location, api_key: &str) -> Result<WeatherResponse> {
        let (lat, lon) = match (location.latitude, location.longitude) {
            (Some(lat), Some(lon)) => (lat, lon),
            _ => anyhow::bail!("Location does not have coordinates"),
        };

        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=metric",
            lat, lon, api_key
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch weather data")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Weather API error: {} - {}", status, body);
        }

        let weather_data: OpenWeatherResponse = response
            .json()
            .await
            .context("Failed to parse weather data")?;

        let description = weather_data
            .weather
            .first()
            .map(|w| w.description.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        Ok(WeatherResponse {
            location_id: location.id,
            location_name: location.name.clone(),
            temperature_celsius: weather_data.main.temp,
            temperature_fahrenheit: celsius_to_fahrenheit(weather_data.main.temp),
            feels_like_celsius: weather_data.main.feels_like,
            humidity: weather_data.main.humidity,
            description,
            wind_speed_ms: weather_data.wind.speed,
            wind_speed_mph: ms_to_mph(weather_data.wind.speed),
        })
    }
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn ms_to_mph(ms: f32) -> f32 {
    ms * 2.237
}
