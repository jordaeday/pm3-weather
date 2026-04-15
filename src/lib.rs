use serde::Deserialize;

pub mod weather;
pub mod time;

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub current: CurrentWeather,
}

#[derive(Debug, Deserialize)]
pub struct CurrentWeather {
    pub time: String,
    #[serde(rename = "temperature_2m")]
    pub temperature: f64,
    pub weather_code: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GeocodingResponse {
    pub results: Vec<GeocodingResult>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GeocodingResult {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "admin1")]
    pub state: String,
}