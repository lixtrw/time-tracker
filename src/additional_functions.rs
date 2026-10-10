use chrono::{Local, NaiveDate};
use color_eyre::eyre::{Context, Ok, Result, eyre};
use std::time::Duration;

pub fn format_to_duration(time: &str) -> Result<Duration> {
    let mut total_duration = Duration::ZERO;
    let mut remainder = time.to_lowercase();
    let mut remainder_str = remainder.as_str();

    while !remainder_str.is_empty() {
        // 1. Ищем, где заканчивается число (первая не-цифра)
        let split_pos = remainder_str
            .find(|c: char| !c.is_ascii_digit())
            .ok_or_else(|| eyre!("There is no unit of measurement in '{}'", remainder_str))?;

        let (num_str, rest) = remainder_str.split_at(split_pos);

        // 2. Ищем, где заканчиваются буквы единицы измерения (следующая цифра или конец строки)
        let end_of_unit = rest
            .find(|c: char| c.is_ascii_digit())
            .unwrap_or(rest.len());

        let (unit_str, next_part) = rest.split_at(end_of_unit);

        // 3. Парсим число
        let value: u64 = num_str
            .parse()
            .wrap_err_with(|| format!("Failed to parse the number: '{}'", num_str))?;

        // 4. Считаем длительность для текущего кусочка
        let part_duration = match unit_str {
            "s" | "sec" | "secs" => Duration::from_secs(value),
            "m" | "min" | "mins" => Duration::from_secs(value * 60),
            "h" | "hour" | "hours" => Duration::from_secs(value * 3600),
            "d" | "day" | "days" => Duration::from_secs(value * 86400),
            "ms" => Duration::from_millis(value),
            _ => return Err(eyre!("Unknown unit of measurement: '{}'", unit_str)),
        };

        total_duration += part_duration;

        // Переходим к следующему фрагменту строки
        remainder_str = next_part;
    }

    Ok(total_duration)
}
pub fn parse_day(day: &str) -> Result<NaiveDate> {
    let day_n = match day {
        "y" => Local::now().date_naive() - chrono::Duration::days(1),
        "t" => Local::now().date_naive(),
        _ => NaiveDate::parse_from_str(day, "%-d-%-m-%Y")?,
    };

    Ok(day_n)
}
