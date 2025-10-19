use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherResponse {
    pub location_id: Uuid,
    pub location_name: String,
    pub temperature_celsius: f32,
    pub temperature_fahrenheit: f32,
    pub feels_like_celsius: f32,
    pub humidity: u8,
    pub description: String,
    pub wind_speed_ms: f32,
    pub wind_speed_mph: f32,
}

#[derive(Debug, Deserialize)]
pub struct OpenWeatherResponse {
    pub main: OpenWeatherMain,
    pub weather: Vec<OpenWeatherCondition>,
    pub wind: OpenWeatherWind,
}

#[derive(Debug, Deserialize)]
pub struct OpenWeatherMain {
    pub temp: f32,
    pub feels_like: f32,
    pub humidity: u8,
}

#[derive(Debug, Deserialize)]
pub struct OpenWeatherCondition {
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenWeatherWind {
    pub speed: f32,
}
