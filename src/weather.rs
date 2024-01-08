pub fn match_condition(cond: &str) -> String {
    match cond {
        "clear" => "\u{2601}\u{FE0F} ясно".to_string(),
        "partly-cloudy " => "малооблачно".to_string(),
        "cloudy" => "облачно с прояснениями".to_string(),
        "overcast" => "\u{2601}\u{FE0F} пасмурно".to_string(),
        "light-rain" => "небольшой дождь".to_string(),
        "rain" => "дождь".to_string(),
        "heavy-rain" => "сильный дождь".to_string(),
        "showers" => "ливень".to_string(),
        "wet-snow" => "дождь со снегом".to_string(),
        "light-snow" => "❄️ небольшой снег".to_string(),
        "snow" => "❄️ снег".to_string(),
        "snow-showers" => "❄️ снегопад".to_string(),
        "hail" => "град".to_string(),
        "thunderstorm" => "гроза".to_string(),
        "thunderstorm-with-rain" => "дождь с грозой".to_string(),
        "thunderstorm-with-hail" => "гроза с градом".to_string(),
        _ => cond.to_string(),
    }
}

pub fn math_condition_int(cond: i64) -> String {
    match cond {
        0 => "\u{2601}\u{FE0F} ясно".to_string(),
        1 | 2 => "малооблачно".to_string(),
        3 => "\u{2601}\u{FE0F} пасмурно".to_string(),
        45 | 48 => "туман".to_string(), // You'll need to provide a string for "Fog and depositing rime fog"
        51 | 53 | 55 => "небольшой дождь".to_string(), // Assuming all drizzle is considered "light rain"
        56 | 57 => "изморозь".to_string(), // You'll need to provide a string for "Freezing Drizzle"
        61 | 63 | 65 => "дождь".to_string(), // Assuming all rain is considered "rain", adjust if necessary
        66 | 67 => "ледяной дождь".to_string(), // You'll need to provide a string for "Freezing Rain"
        71 | 73 | 75 => "❄️ снег".to_string(), // Assuming all snow fall is considered "snow"
        77 => "снежные зерна".to_string(), // You'll need to provide a string for "Snow grains"
        80 | 81 | 82 => "ливень".to_string(),
        85 | 86 => "❄️ снегопад".to_string(),
        95 => "гроза".to_string(),
        96 | 99 => "гроза с градом".to_string(),
        _ => cond.to_string(),
    }
}