use slint::Color;

pub enum WeatherCode {
  ClearSky, // 0
  MostlyClear, // 1
  PartlyCloudy, // 2
  Overcast, // 3
  Fog, // 45, 48
  Drizzle, // 51, 53, 55
  FreezingDrizzle, // 56, 57
  Rain, // 61, 63, 65, 80, 81, 82
  FreezingRain, // 66, 67
  Snowfall, // 71, 73, 75, 77, 85, 86
  Thunderstorm, // 95, 96, 99
}

pub enum Effect {
  Rain,
  Snow,
  Lightning,
  Fog
}

impl WeatherCode {
  pub fn from_code(code: i32) -> Self {
    match code {
      0 => WeatherCode::ClearSky,
      1 => WeatherCode::MostlyClear,
      2 => WeatherCode::PartlyCloudy,
      3 => WeatherCode::Overcast,
      45 | 48 => WeatherCode::Fog,
      51 | 53 | 55 => WeatherCode::Drizzle,
      56 | 57 => WeatherCode::FreezingDrizzle,
      61 | 63 | 65 | 80 | 81 | 82 => WeatherCode::Rain,
      66 | 67 => WeatherCode::FreezingRain,
      71 | 73 | 75 | 77 | 85 | 86 => WeatherCode::Snowfall,
      95 | 96 | 99 => WeatherCode::Thunderstorm,
      _ => WeatherCode::ClearSky, // Default case for unknown codes
    }
  }

  pub fn description_name(&self) -> &'static str {
    match self {
      WeatherCode::ClearSky => "Clear Sky",
      WeatherCode::MostlyClear => "Mostly Clear",
      WeatherCode::PartlyCloudy => "Partly Cloudy",
      WeatherCode::Overcast => "Overcast",
      WeatherCode::Fog => "Fog",
      WeatherCode::Drizzle => "Drizzle",
      WeatherCode::FreezingDrizzle => "Freezing Drizzle",
      WeatherCode::Rain => "Rain",
      WeatherCode::FreezingRain => "Freezing Rain",
      WeatherCode::Snowfall => "Snowfall",
      WeatherCode::Thunderstorm => "Thunderstorm",
    }
  }

  pub fn effects(&self) -> &[Effect] {
    match self {
      WeatherCode::ClearSky => &[],
      WeatherCode::MostlyClear => &[],
      WeatherCode::PartlyCloudy => &[],
      WeatherCode::Overcast => &[],
      WeatherCode::Fog => &[Effect::Fog],
      WeatherCode::Drizzle => &[Effect::Rain],
      WeatherCode::FreezingDrizzle => &[Effect::Rain, Effect::Snow],
      WeatherCode::Rain => &[Effect::Rain],
      WeatherCode::FreezingRain => &[Effect::Rain, Effect::Snow],
      WeatherCode::Snowfall => &[Effect::Snow],
      WeatherCode::Thunderstorm => &[Effect::Rain, Effect::Lightning],
    }
  }

  pub fn sky_color(&self) -> Color {
    match self {
      WeatherCode::ClearSky | WeatherCode::MostlyClear
        => Color::from_rgb_u8(100, 160, 230),
      WeatherCode::PartlyCloudy
        => Color::from_rgb_u8(120, 150, 190),
      WeatherCode::Overcast
        => Color::from_rgb_u8(90, 90, 100),
      WeatherCode::Fog
        => Color::from_rgb_u8(150, 150, 160),
      WeatherCode::Drizzle | WeatherCode::Rain
        => Color::from_rgb_u8(60, 80, 110),
      WeatherCode::FreezingDrizzle | WeatherCode::FreezingRain 
        => Color::from_rgb_u8(80, 100, 130),
      WeatherCode::Snowfall
        => Color::from_rgb_u8(180, 190, 210),
      WeatherCode::Thunderstorm
        => Color::from_rgb_u8(30, 30, 50),
    }
  }
}