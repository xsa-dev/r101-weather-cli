use clap::Parser;
use yandex_weather::YandexWeatherApi;
use owheater_onecall::WeatherData;
use crate::open_meteo::OpenMeteo;

mod weather;
mod owheater_onecall;
mod yandex_weather;
mod coloring;
mod open_meteo;


#[derive(Debug, Parser)]
#[command(about, version, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    forecast: Option<usize>,
}

fn main() -> anyhow::Result<()> {
    if cfg!(debug_assertions) {
        dotenvy::dotenv()?;
    }

    let api_key = match std::env::var("YANDEX_WEATHER_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("YANDEX_WEATHER_API_KEY не найден");
            std::process::exit(1)
        }
    };
    let open_api_key = match std::env::var("OPEN_WEATHER_MAP_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("OPEN_WEATHER_MAP_API_KEY не найден");
            std::process::exit(1)
        }
    };

    let lat = std::env::var("LAT_COORD").unwrap_or("55.75396".to_string());
        let lon = std::env::var("LON_COORD").unwrap_or("37.620393".to_string());

    let cli = Cli::parse();

    let periods: i32 = cli.forecast.unwrap_or(0) as i32;

    if periods == 0 {
        YandexWeatherApi::new(&api_key, &lat, &lon, &periods)?.display_now()?;
    } else if (1..=2).contains(&periods) {
        YandexWeatherApi::new(&api_key, &lat, &lon, &periods)?
            .display_now()?
            .display_forecast()?;
    }
    else if (3..=7).contains(&periods) {
        OpenMeteo::new(&lat, &lon, &periods)?
            .display_now()?
            .display_forecast()?;
    }
    else if 8.eq(&periods) {
        WeatherData::new(&open_api_key, &lat, &lon)?
            .display_now()
            .unwrap().display_forecast()?;
    } else if (8..=16).contains(&periods) {
        OpenMeteo::new(&lat, &lon, &periods)?
            .display_now()?
            .display_forecast()?;
    } else {
        eprintln!("Период прогноза {} не поддерживается. Поддерживаются периоды от 0 до 16.", periods);
    }

    Ok(())
}
