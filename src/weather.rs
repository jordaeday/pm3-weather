use reqwest;
use crate::{GeocodingResponse, GeocodingResult, WeatherResponse};

fn find_location(city_name: &str, region: Option<&str>) -> Result<GeocodingResult, Box<dyn std::error::Error>> {
    let count = if region.is_some() { 10 } else { 1 };
    let geocoding_url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count={}",
        city_name, count
    );
    let geocoding_response: GeocodingResponse = reqwest::blocking::get(&geocoding_url)?.json()?;

    if let Some(region) = region {
        geocoding_response.results.into_iter()
            .find(|r| r.state.as_deref().map(|s| s.eq_ignore_ascii_case(region)).unwrap_or(false))
            .ok_or_else(|| format!("No result for '{}' in '{}'", city_name, region).into())
    } else {
        geocoding_response.results.into_iter().next().ok_or("City not found in geocoding API".into())
    }
}

pub fn get_weather_from_city(city_name: &str, region: Option<&str>) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
    let location = find_location(city_name, region)?;
    let weather_url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,weather_code&temperature_unit=fahrenheit&precipitation_unit=inch",
        location.latitude, location.longitude
    );
    let weather_response: WeatherResponse = reqwest::blocking::get(&weather_url)?.json()?;
    Ok(weather_response)
}

pub fn get_state_from_city(city_name: &str, region: Option<&str>) -> Result<GeocodingResult, Box<dyn std::error::Error>> {
    find_location(city_name, region)
}