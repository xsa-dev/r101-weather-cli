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