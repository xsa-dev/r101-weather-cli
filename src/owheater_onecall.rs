use chrono::{Datelike, DateTime, NaiveDate, Utc};
use serde::Deserialize;
use crate::coloring::with_gray;
use crate::weather::match_condition;
use chrono::NaiveDateTime;

#[derive(Debug, Deserialize)]
pub(crate) struct WeatherData {
    lat: f64,
    lon: f64,
    timezone: String,
    timezone_offset: i32,
    current: CurrentWeather,
    daily: Vec<DailyData>,
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    dt: i64,
    sunrise: i64,
    sunset: i64,
    temp: f64,
    feels_like: f64,
    pressure: i32,
    humidity: i32,
    dew_point: f64,
    uvi: f64,
    clouds: i32,
    visibility: i32,
    wind_speed: f64,
    wind_deg: i32,
    wind_gust: f64,
    weather: Vec<WeatherDescription>,
}

#[derive(Debug, Deserialize)]
struct WeatherDescription {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Deserialize)]
struct DailyData {
    dt: i64,
    sunrise: i64,
    sunset: i64,
    moonrise: i64,
    moonset: i64,
    moon_phase: f64,
    temp: Temperature,
    feels_like: FeelsLike,
    pressure: i32,
    humidity: i32,
    dew_point: f64,
    wind_speed: f64,
    wind_deg: i32,
    wind_gust: f64,
    weather: Vec<WeatherDescription>,
    clouds: i32,
    pop: f64,
    snow: Option<f64>,
    uvi: f64,
}

#[derive(Debug, Deserialize)]
struct Temperature {
    day: f64,
    min: f64,
    max: f64,
    night: f64,
    eve: f64,
    morn: f64,
}

#[derive(Debug, Deserialize)]
struct FeelsLike {
    day: f64,
    night: f64,
    eve: f64,
    morn: f64,
}


impl WeatherData {
    #[must_use = "Примените методы структуры"]
    pub fn new(open_api_key: &str, lat: &str, lon: &str) -> anyhow::Result<Self> {

        let response = reqwest::blocking::Client::new()
            .get("https://api.openweathermap.org/data/2.5/onecall")
            .query(&[
                ("lat", lat), ("lon", lon),
                ("appid", open_api_key),
                ("units", "metric"),
                ("lang", "ru"),
                ("exclude", "minutely,hourly"),
                ("appid", open_api_key)])
            .send()?;

            if response.status().is_success() {
                let response_text = response.text()?;
                let data: Self = serde_json::from_str(&response_text)?;
                Ok(data)
            } else {
                println!("Ошибка запроса: {}", response.status());
                Err(anyhow::Error::msg("Request error"))
            }
    }

    pub fn display_now(self) -> anyhow::Result<Self> {
        let date = DateTime::<Utc>::from_timestamp(self.current.dt, 0).unwrap();

        let header = format!("--- Сейчас ({}) ---", date.format("%a, %d %b %Y"));

        println!("{}", with_gray(&header));
        println!(
            "{}\u{00B0}C (ощущается как {}\u{00B0}C)",
            self.current.temp, self.current.feels_like
        );
        println!("{}", match_condition(&self.current.weather[0].description.replace(' ', "-")), );

        Ok(self)
    }

    pub fn display_forecast(self) -> anyhow::Result<()> {
        let date = DateTime::<Utc>::from_timestamp(self.current.dt, 0)
            .unwrap()
            .date_naive();

        println!("{}", with_gray(&format!("--- Прогноз на {} дней ({}) ---", self.daily.len(), date.format("%a, %d %b %Y"))));

        for f in self.daily {
            let current_datetime = NaiveDateTime::from_timestamp_opt(f.dt, 0).unwrap();
            let current_date = NaiveDate::from_ymd_opt(current_datetime.year(), current_datetime.month(), current_datetime.day());

            let header = format!("\n--- {} (прогноз от: {}) ---", current_date.unwrap().format("%a, %d %b %Y"), date.format("%a, %d %b %Y"));

            println!("{}", with_gray(&header));
            println!("В течении дня: {}..{}\u{00B0}C по погоде: {}.", f.temp.min, f.temp.max, match_condition(&f.weather[0].description.replace(' ', "-")));
            println!("{} {}\u{00B0}C ощущается как {}\u{00B0}C", "Утро:", f.temp.morn, f.feels_like.morn);
            println!("{} {}\u{00B0}C ощущается как {}\u{00B0}C", "День:", f.temp.day, f.feels_like.day);
            println!("{} {}\u{00B0}C ощущается как {}\u{00B0}C", "Вечер:", f.temp.eve, f.feels_like.eve);
            println!("{} {}\u{00B0}C ощущается как {}\u{00B0}C", "Ночь:", f.temp.night, f.feels_like.night);
        }

        Ok(())
    }
}

