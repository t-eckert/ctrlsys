use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GeocodingResult {
    pub city_name: String,
    pub country: String,
    pub state: Option<String>,
    pub latitude: f32,
    pub longitude: f32,
    pub timezone: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenWeatherGeoResponse {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub country: String,
    pub state: Option<String>,
}
