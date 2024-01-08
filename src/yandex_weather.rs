use chrono::{DateTime, Duration, Utc};
use std::ops::{Deref};
use serde::Deserialize;
use serde_json::Value;


use crate::weather::match_condition;
use crate::coloring::with_gray;

#[derive(Deserialize, Debug)]
pub struct YandexWeatherApi {
    #[serde(rename = "now")]
    now_timestamp: i64,
    fact: FactWeather,
    forecast: Value,
}

#[derive(Deserialize, Debug)]
pub(crate) struct FactWeather {
    pub(crate) condition: String,
    pub(crate) temp: i32,
    pub(crate) feels_like: i32,
}


#[derive(Deserialize)]
pub(crate) struct ForecastDay {
    pub(crate) temp_min: i32,
    pub(crate) temp_max: i32,
    pub(crate) part_name: String,
    pub(crate) condition: String,
}


impl YandexWeatherApi {
    #[must_use = "Примените методы структуры"]
    pub fn new(api_key: &str, lat: &str, lon: &str, periods: &i32) -> anyhow::Result<Self> {
        let days: String = periods.to_string();

        let response = reqwest::blocking::Client::new()
            .get("https://api.weather.yandex.ru/v2/informers")
            .query(&[("lat", lat), ("lon", lon), ("lang", "ru_RU"), ("limit", &days)])
            .header("X-Yandex-API-Key", api_key)
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
        let date = DateTime::<Utc>::from_timestamp(self.now_timestamp, 0).unwrap();

        let header = format!("--- Сейчас ({}) ---", date.format("%a, %d %b %Y"));

        println!("{}", with_gray(&header));
        println!(
            "{}\u{00B0}C (ощущается как {}\u{00B0}C)",
            self.fact.temp, self.fact.feels_like
        );
        println!("{}", match_condition(&self.fact.condition), );

        Ok(self)
    }

    pub fn display_forecast(self) -> anyhow::Result<()> {
        let mut date = DateTime::<Utc>::from_timestamp(self.now_timestamp, 0)
            .unwrap()
            .date_naive();

        let forecast_data: Vec<ForecastDay> =
            serde_json::from_value(self.forecast["parts"].clone())?;

        for f in forecast_data {
            if f.part_name == "night" {
                date += Duration::days(1);
            }

            let part_name = match f.part_name.deref() {
                "night" => "Ночь",
                "morning" => "Утро",
                "day" => "День",
                "evening" => "Вечер",
                _ => "",
            };

            let header = format!("\n--- {} ({}) ---", part_name, date.format("%a, %d %b %Y"));

            println!("{}", with_gray(&header));
            println!("{}..{}\u{00B0}C", f.temp_min, f.temp_max);
            println!("{}", match_condition(&f.condition));
        }

        Ok(())
    }
}

