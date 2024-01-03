use clap::Parser;

mod forecast;
mod weather;

use crate::forecast::{OpenWeatherMapApi, YandexWeatherApi};

#[derive(Debug, Parser)]
#[command(about, version, long_about = None)]
struct Cli {
    /// Вывести прогноз погоды
    #[arg(short, long)]
    /// Вывести прогноз погоды на указанное количество периодов (по умолчанию 1)
    forecast: Option<usize>,
}

fn main() -> anyhow::Result<()> {
    // conditional compilation
    if cfg!(debug_assertions) {
        // файл .env будет запрашиваться только из debug-бинарника
        dotenvy::dotenv()?;
    }

    // попытаемся получить значение переменной окружения YANDEX_WEATHER_API_KEY
    let api_key = match std::env::var("YANDEX_WEATHER_API_KEY") {
        // вернем ключ, если от есть
        Ok(key) => key,
        Err(_) => {
            // если ключа нет, то выведем сообщение в stderr и завершим программу
            eprintln!("YANDEX_WEATHER_API_KEY не найден");
            std::process::exit(1)
        }
    };
    // попытаемся получить значение переменной окружения OPEN_WEATHER_MAP_API_KEY
    let open_api_key = match std::env::var("OPEN_WEATHER_MAP_API_KEY") {
        // вернем ключ, если от есть
        Ok(key) => key,
        Err(_) => {
            eprintln!("OPEN_WEATHER_MAP_API_KEY не найден");
            std::process::exit(1)
        }
    };

    // если не заданы переменные окружения `LAT_COORD` и `LON_COORD`,
    // то будем использовать координаты Москвы
    let lat = std::env::var("LAT_COORD").unwrap_or("55.75396".to_string());
    let lon = std::env::var("LON_COORD").unwrap_or("37.620393".to_string());

    // получим аргументы, переданные нам при запуске утилиты
    let cli = Cli::parse();

    // получим значение периода прогноза
    let periods: usize = cli.forecast.unwrap_or(0);

    // если periods равен 0, то вывести прогноз на сегодня
    if periods == 0 {
        YandexWeatherApi::new(&api_key, &lat, &lon)?.display_now()?;
    } else if (1..=2).contains(&periods) {
        // если periods от 1 до 2 (включительно), то вывести прогноз на сегодня и на periods периодов вперед
        YandexWeatherApi::new(&api_key, &lat, &lon)?
            .display_now()?
            .display_forecast()?;
    } else if (3..=16).contains(&periods) {
        // если periods от 3 до 16 (включительно), то вывести прогноз на сегодня и на periods периодов вперед
        OpenWeatherMapApi::new(&open_api_key, &lat, &lon)?
            .display_now()?
            .display_forecast()?;
    } else {
        // если periods не соответствует ни одному из заданных условий, то вывести сообщение о не поддерживаемом периоде
        eprintln!("Период прогноза {} не поддерживается. Поддерживаются периоды от 0 до 16.", periods);
    }

    Ok(())
}
