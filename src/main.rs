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

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("Usage: weather <city-name> <state-name>");
        std::process::exit(1);
    }
    let city_name = args[0].clone();
    let state_name = args[1].clone();

    #[cfg(feature = "framebuffer")] {
        use slint_backend_linuxfb::LinuxFbPlatformBuilder;

        let platform = LinuxFbPlatformBuilder::new()
            .with_framebuffer("/dev/fb0")
            .with_input_autodiscovery(true)
            .build()
            .unwrap();

        slint::platform::set_platform(Box::new(platform)).unwrap();
    }

    // For getting weather, first get gecoords, then weather
    // geocoding api
    // hardcode for now, update later
    let LATITUDE: f64 = 47.60621;
    let LONGITUDE: f64 = -122.33207;

    // weather api
    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,weather_code&wind_speed_unit=mph&temperature_unit=fahrenheit&timezone=auto", LATITUDE, LONGITUDE);
    let response = reqwest::blocking::get(&url)?.json::<WeatherResponse>()?;
    println!("Current temperature in {}, {}: {}°F, weather code: {}", city_name, state_name, response.current.temperature, response.current.weather_code);

    let main_window = MainWindow::new()?;

    main_window.set_temperature(response.current.temperature as f32);
    main_window.set_weather_code(response.current.weather_code);
    main_window.set_city_name(city_name.into());

    main_window.run()?;
    Ok(())
}