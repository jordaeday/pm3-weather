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
        => Color::from_rgb_u8(0x4A, 0x90, 0xD9), // #4A90D9
      WeatherCode::PartlyCloudy
        => Color::from_rgb_u8(0x7A, 0x9D, 0xC0), // #7A9DC0
      WeatherCode::Overcast
        => Color::from_rgb_u8(0x8A, 0x8D, 0x9C), // #8A8D9C
      WeatherCode::Fog
        => Color::from_rgb_u8(0x9D, 0xA0, 0xAB), // #9DA0AB
      WeatherCode::Drizzle | WeatherCode::Rain
        => Color::from_rgb_u8(0x5A, 0x70, 0x90), // #5A7090
      WeatherCode::FreezingDrizzle | WeatherCode::FreezingRain
        => Color::from_rgb_u8(0x6A, 0x85, 0xA0), // #6A85A0
      WeatherCode::Snowfall
        => Color::from_rgb_u8(0xA0, 0xB8, 0xCC), // #A0B8CC
      WeatherCode::Thunderstorm
        => Color::from_rgb_u8(0x45, 0x45, 0x60), // #454560
    }
  }
}