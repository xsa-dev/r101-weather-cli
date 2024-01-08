use chrono::{DateTime, Duration, Utc};
use std::ops::Deref;
use crate::forecast;
use crate::forecast::{ForecastDay, YandexWeatherApi};
use crate::weather::match_condition;

impl YandexWeatherApi {
    #[must_use = "Примените методы структуры"]
    pub fn new(api_key: &str, lat: &str, lon: &str, days: &i32) -> anyhow::Result<Self> {
        /*
        TODO implement
        TODO посмотреть нет ли параметра для получения прогноза на несколько дней:
        TODO limit - кол-во дней (для тарифа тестовый 7 дней)
        https://yandex.ru/dev/weather/doc/dg/concepts/forecast-test.html
        */
        let response = reqwest::blocking::Client::new()
            .get("https://api.weather.yandex.ru/v2/informers")
            .query(&[("lat", lat), ("lon", lon), ("lang", "ru_RU"), ("limit", stringify!(days))])
            .header("X-Yandex-API-Key", api_key)
            .send()?;

        if response.status().is_success() {
            let response_text = response.text()?;
            let data: Self = serde_json::from_str(&response_text)?;
            println!("{:?}", data);
            Ok(data)
        } else {
            println!("Ошибка запроса: {}", response.status());
            Err(anyhow::Error::msg("Request error"))
        }

        // Также можно
        // let data: Self = serde_json::from_str(&response.text()?)?;
        // println!("{:?}", data);
    }

    pub fn display_now(self) -> anyhow::Result<Self> {
        let date = DateTime::<Utc>::from_timestamp(self.now_timestamp, 0).unwrap();

        let header = format!("--- Сейчас ({}) ---", date.format("%a, %d %b %Y"));

        println!("{}", forecast::with_gray(&header));
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

            println!("{}", forecast::with_gray(&header));
            println!("{}..{}\u{00B0}C", f.temp_min, f.temp_max);
            println!("{}", match_condition(&f.condition));
        }

        Ok(())
    }
}
