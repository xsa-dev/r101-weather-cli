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

    let periods: usize = cli.forecast.unwrap_or(0);

    if periods > 0 && periods <= 2 {
        // если флаг `-f` был передан, то выведем еще и прогноз на periods периода вперед (до 2 периодов)
        YandexWeatherApi::new(&api_key, &lat, &lon)?
            .display_now()?
            .display_forecast(periods)?;
    } else if periods > 2 && periods <= 16 {
        // если флаг `-f` был передан, то выведем еще и прогноз на 2-16 периода вперед. (от 2 до 16 периодов)
        OpenWeatherMapApi::new(&open_api_key, &lat, &lon)?
            .display_now()?
            .display_forecast(periods)?;
    } else {
        // если флага `-f` нет, то выведем погоду на текущее время
        YandexWeatherApi::new(&api_key, &lat, &lon)?.display_now()?;
        if periods > 16 {
            eprintln!("Для указанного количества периодов прогноза погоды нет");
        }
    }

    Ok(())
}
