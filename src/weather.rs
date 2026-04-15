use reqwest;
use crate::{GeocodingResponse, GeocodingResult, WeatherResponse};

pub fn get_weather_from_city(city_name: &str) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
    let geocoding_url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1",
        city_name
    );
    let geocoding_response: GeocodingResponse = reqwest::blocking::get(&geocoding_url)?.json()?;
    let location = geocoding_response
        .results
        .get(0)
        .ok_or("City not found in geocoding API")?;

    let weather_url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,weather_code",
        location.latitude, location.longitude
    );
    let weather_response: WeatherResponse = reqwest::blocking::get(&weather_url)?.json()?;
    Ok(weather_response)
}

pub fn get_state_from_city(city_name: &str) -> Result<GeocodingResult, Box<dyn std::error::Error>> {
    let geocoding_url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1",
        city_name
    );
    let geocoding_response: GeocodingResponse = reqwest::blocking::get(&geocoding_url)?.json()?;
    let location = geocoding_response
        .results
        .get(0)
        .ok_or("City not found in geocoding API")?;
    Ok(location.clone())
}