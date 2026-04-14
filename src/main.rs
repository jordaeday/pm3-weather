slint::include_modules!();
use serde::Deserialize;
use reqwest;

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    current: CurrentWeather,
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    time: String,
    #[serde(rename = "temperature_2m")]
    temperature: f64,
    weather_code: i32,
}

#[derive(Debug, Deserialize)]
struct GeocodingResponse {
    results: Vec<GeocodingResult>,
}

#[derive(Debug, Deserialize)]
struct GeocodingResult {
    latitude: f64,
    longitude: f64,
    #[serde(rename = "admin1")]
    state: String,
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 1 {
        eprintln!("Usage: weather <city-name>");
        std::process::exit(1);
    }
    let city_name = capitalize(&args[0]);

    #[cfg(feature = "framebuffer")] {
        use slint_backend_linuxfb::LinuxFbPlatformBuilder;

        let platform = LinuxFbPlatformBuilder::new()
            .with_framebuffer("/dev/fb0")
            .with_input_autodiscovery(true)
            .build()
            .unwrap();

        slint::platform::set_platform(Box::new(platform)).unwrap();
    }

    // For getting weather, first get gecoords, then use those for weather
    // geocoding api
    let geocoding_url = format!("https://geocoding-api.open-meteo.com/v1/search?name={}", city_name);
    let geocoding_response = reqwest::blocking::get(&geocoding_url)?.json::<GeocodingResponse>()?;
    let latitude: f64 = geocoding_response.results[0].latitude;
    let longitude: f64 = geocoding_response.results[0].longitude;
    let state_name = &geocoding_response.results[0].state;

    println!("Coordinates for {}, {}: {}, {}", city_name, state_name, latitude, longitude);

    // weather api
    let weather_url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,weather_code&wind_speed_unit=mph&temperature_unit=fahrenheit&timezone=auto", latitude, longitude);
    let response = reqwest::blocking::get(&weather_url)?.json::<WeatherResponse>()?;
    println!("Current temperature in {}, {}: {}°F, weather code: {}", city_name, state_name, response.current.temperature, response.current.weather_code);

    let main_window = MainWindow::new()?;

    main_window.set_temperature(response.current.temperature as f32);
    main_window.set_weather_code(response.current.weather_code);
    main_window.set_city_name(city_name.into());
    main_window.set_state_name(state_name.clone().into());
    main_window.run()?;
    Ok(())
}