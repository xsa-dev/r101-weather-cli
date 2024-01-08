use serde::{Deserialize};
use crate::coloring::with_gray;
use crate::weather::math_condition_int;

#[derive(Deserialize, Debug)]
pub struct OpenMeteo {
    latitude: f64,
    longitude: f64,
    generationtime_ms: f64,
    utc_offset_seconds: i64,
    timezone: String,
    timezone_abbreviation: String,
    elevation: f64,
    current_units: Units,
    current: Current,
    daily_units: Units,
    daily: Daily,
}

#[derive(Deserialize, Debug)]
pub struct Units {
    time: String,
    #[serde(rename = "interval")]
    interval_units: Option<String>,
    #[serde(rename = "temperature_2m")]
    temperature_2m_units: Option<String>,
    weather_code: String,
    #[serde(rename = "temperature_2m_max")]
    temperature_2m_max_units: Option<String>,
    #[serde(rename = "temperature_2m_min")]
    temperature_2m_min_units: Option<String>,
    #[serde(rename = "apparent_temperature_max")]
    apparent_temperature_max_units: Option<String>,
    #[serde(rename = "apparent_temperature_min")]
    apparent_temperature_min_units: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Current {
    time: String,
    interval: i64,
    temperature_2m: f64,
    weather_code: i64,
}

#[derive(Deserialize, Debug)]
pub struct Daily {
    time: Vec<String>,
    weather_code: Vec<i64>,
    temperature_2m_max: Vec<f64>,
    temperature_2m_min: Vec<f64>,
    apparent_temperature_max: Vec<f64>,
    apparent_temperature_min: Vec<f64>,
}

// Usage example (assuming you have a JSON string named `json_str`):
// let open_meteo: OpenMeteo = serde_json::from_str(json_str)?;


impl OpenMeteo {
    #[must_use = "Примените методы структуры"]
    pub fn new(lat: &str, lon: &str, periods: &i32) -> anyhow::Result<Self> {
        let days: String = periods.to_string();

        let response = reqwest::blocking::Client::new()
            .get("https://api.open-meteo.com/v1/forecast")
            .query(&[
                ("latitude", lat), ("longitude", lon),
                ("current", "temperature_2m,weather_code"),
                ("daily", "weather_code,temperature_2m_max,temperature_2m_min,apparent_temperature_max,apparent_temperature_min"),
                ("timezone", "Europe/Moscow"),
                ("forecast_days", &days),
                ("temperature_unit", "celsius")]
            ).send()?;


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
        let header = format!("--- Сейчас {} ---", self.current.time);
        println!("{}", with_gray(&header));
        println!(
            "{}C\u{00B0}",
            self.current.temperature_2m
        );
        println!("{}", math_condition_int(self.current.weather_code));
        Ok(self)
    }

    pub fn display_forecast(self) -> anyhow::Result<()> {
        let now = self.current.time.clone();
        println!("{}", with_gray(&format!("--- Прогноз на {} дней от {} ---", self.daily.time.len(), now
        )));
        //
        for (index, _f) in self.daily.time.iter().enumerate() {
            let header: String = format!("\n--- {} (прогноз от: {}) ---", self.daily.time[index], now);
            println!("{}", with_gray(&header));
            println!(
                "В течении дня: от {} \u{00B0}C до {} \u{00B0}C.\r\nБудет ощущаться как: {} \u{00B0}C и {} \u{00B0}C.", self.daily.temperature_2m_min[index], self.daily.temperature_2m_max[index], self.daily.apparent_temperature_min[index], self.daily.apparent_temperature_max[index]
            );
            println!(
                "По погоде: {}", math_condition_int(self.daily.weather_code[index])
            )
        }

        Ok(())
    }
}

