slint::include_modules!();
use chrono::Timelike;
use pm3_weather::weather_code::WeatherCode;
use slint::{Color, Timer, TimerMode};

use pm3_weather::time::sync_time_from_ntp;
use pm3_weather::weather::{get_weather_from_city, get_state_from_city};

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn time_of_day_color(hour: u32) -> slint::Color {
    match hour {
        0..=5 => Color::from_rgb_u8(0x00, 0x00, 0x20),   // Night #000020
        6..=11 => Color::from_rgb_u8(0xFF, 0xA5, 0x00),  // Morning #FFA500
        12..=17 => Color::from_rgb_u8(0x87, 0xCE, 0xEB), // Afternoon #87CEEB
        18..=19 => Color::from_rgb_u8(0xFF, 0x45, 0x00), // Evening #FF4500
        20..=23 => Color::from_rgb_u8(0x00, 0x00, 0x20), // Night #000020
        _ => Color::from_rgb_u8(0x00, 0x00, 0x20),       // Default to night #000020
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 1 {
        eprintln!("Usage: weather <city-name> [state/region]");
        std::process::exit(1);
    }
    let city_name = capitalize(&args[0]);
    let region: Option<String> = args.get(1).map(|s| capitalize(s));

    #[cfg(feature = "framebuffer")] {
        use slint_backend_linuxfb::LinuxFbPlatformBuilder;

        let platform = LinuxFbPlatformBuilder::new()
            .with_framebuffer("/dev/fb0")
            .with_input_autodiscovery(true)
            .build()
            .unwrap();

        slint::platform::set_platform(Box::new(platform)).unwrap();
    }

    let main_window = MainWindow::new()?;

    main_window.set_city_name(city_name.clone().into());
    let location = get_state_from_city(&city_name, region.as_deref())?;
    let display_region = region.clone()
        .or_else(|| location.state.clone())
        .unwrap_or_default();
    main_window.set_state_name(display_region.into());

    let window_weak = main_window.as_weak();

    // Update system time from NTP every 5 minutes
    if let Err(e) = sync_time_from_ntp() {
        eprintln!("Failed to get time from NTP: {}", e);
    }
    let ntp_timer = Timer::default();
    ntp_timer.start(TimerMode::Repeated, std::time::Duration::from_secs(300), move || {
        if let Err(e) = sync_time_from_ntp() {
            eprintln!("Failed to get time from NTP: {}", e);
        }
    });

    // Get weather every 15 minutes
    let weather_response = get_weather_from_city(&city_name, region.as_deref())?;
    main_window.set_temperature(weather_response.current.temperature as f32);
    main_window.set_weather_code(weather_response.current.weather_code);
    main_window.set_conditions(WeatherCode::from_code(weather_response.current.weather_code).description_name().into());

    // Set initial sky colors
    let code = WeatherCode::from_code(weather_response.current.weather_code);
    let now = chrono::Local::now();
    main_window.set_sky_top(code.sky_color());
    main_window.set_sky_bottom(time_of_day_color(now.hour()));

    let window_weak_weather = window_weak.clone();
    let weather_timer = Timer::default();
    let region_for_timer = region;
    weather_timer.start(TimerMode::Repeated, std::time::Duration::from_secs(900), move || {
        match get_weather_from_city(&city_name, region_for_timer.as_deref()) {
            Ok(response) => {
                if let Some(window) = window_weak_weather.upgrade() {
                    window.set_temperature(response.current.temperature as f32);
                    window.set_weather_code(response.current.weather_code);
                    let code = WeatherCode::from_code(response.current.weather_code);
                    window.set_conditions(code.description_name().into());
                }
            }
            Err(e) => eprintln!("Failed to get weather: {}", e),
        }
    });

    // Update display time from system clock every second
    let window_weak_time = window_weak.clone();
    let time_timer = Timer::default();
    time_timer.start(TimerMode::Repeated, std::time::Duration::from_secs(1), move || {
        if let Some(window) = window_weak_time.upgrade() {
            let now = chrono::Local::now();
            window.set_time(now.format("%H:%M:%S").to_string().into());
            window.set_date(now.format("%A, %B %-d, %Y").to_string().into());
        }
    });

    main_window.run()?;
    Ok(())
}