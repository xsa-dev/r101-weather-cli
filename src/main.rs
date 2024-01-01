use clap::Parser;

mod forecast;
mod weather;
use crate::forecast::YandexWeatherApi;

#[derive(Debug, Parser)]
#[command(about, version, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    /// Вывести прогноз погоды
    forecast: bool,
}

fn main() -> anyhow::Result<()> {
    // conditional compilation
    if cfg!(debug_assertions) {
        // файл .env будет запрашиваться только из debug-бинарника
        dotenvy::dotenv()?;
    }

    let api_key = match std::env::var("YANDEX_WEATHER_API_KEY") {
        Ok(key) => key,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    };

    // если не заданы переменные окружения `LAT_COORD` и `LON_COORD`,
    // то будем использовать координаты Москвы
    let lat = std::env::var("LAT_COORD").unwrap_or("55.75396".to_string());
    let lon = std::env::var("LON_COORD").unwrap_or("37.620393".to_string());

    let cli = Cli::parse();

    if !cli.forecast {
        YandexWeatherApi::new(&api_key, &lat, &lon)?.display_now()?;
    } else {
        YandexWeatherApi::new(&api_key, &lat, &lon)?
            .display_now()?
            .display_forecast()?;
    }

    Ok(())
}
