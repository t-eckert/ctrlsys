use anyhow::{Context, Result};

use crate::models::geocoding::{GeocodingResult, OpenWeatherGeoResponse};

pub struct GeocodingService;

impl GeocodingService {
    /// Lookup location data from city name using OpenWeatherMap Geocoding API
    pub async fn lookup_city(city_name: &str, api_key: &str) -> Result<GeocodingResult> {
        let url = format!(
            "http://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}",
            urlencoding::encode(city_name),
            api_key
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch geocoding data")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Geocoding API error: {} - {}", status, body);
        }

        let geo_data: Vec<OpenWeatherGeoResponse> = response
            .json()
            .await
            .context("Failed to parse geocoding data")?;

        let geo = geo_data
            .first()
            .context(format!("City '{}' not found", city_name))?;

        // Determine timezone from coordinates using tzf-rs
        let finder = tzf_rs::DefaultFinder::new();
        let timezone = finder.get_tz_name(geo.lon, geo.lat);

        Ok(GeocodingResult {
            city_name: geo.name.clone(),
            country: geo.country.clone(),
            state: geo.state.clone(),
            latitude: geo.lat as f32,
            longitude: geo.lon as f32,
            timezone: timezone.to_string(),
        })
    }
}
