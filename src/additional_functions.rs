#![allow(dead_code)]

use chrono::{Datelike, Local, NaiveDate, NaiveTime};
use color_eyre::eyre::{Context, Ok, Result, eyre};
use std::{fs, path::PathBuf, time::Duration};

use crate::activity::{All, Day};
use crate::WeekType;

pub fn format_to_duration(time: &str) -> Result<Duration> {
    let mut total_duration = Duration::ZERO;
    let remainder = time.to_lowercase();
    let mut remainder_str = remainder.as_str();

    while !remainder_str.is_empty() {
        let split_pos = remainder_str
            .find(|c: char| !c.is_ascii_digit())
            .ok_or_else(|| eyre!("There is no unit of measurement in '{}'", remainder_str))?;

        let (num_str, rest) = remainder_str.split_at(split_pos);

        let end_of_unit = rest
            .find(|c: char| c.is_ascii_digit())
            .unwrap_or(rest.len());

        let (unit_str, next_part) = rest.split_at(end_of_unit);

        let value: u64 = num_str
            .parse()
            .wrap_err_with(|| format!("Failed to parse the number: '{}'", num_str))?;

        let part_duration = match unit_str {
            "s" | "sec" | "secs" => Duration::from_secs(value),
            "m" | "min" | "mins" => Duration::from_secs(value * 60),
            "h" | "hour" | "hours" => Duration::from_secs(value * 3600),
            "d" | "day" | "days" => Duration::from_secs(value * 86400),
            "ms" => Duration::from_millis(value),
            _ => return Err(eyre!("Unknown unit of measurement: '{}'", unit_str)),
        };

        total_duration += part_duration;

        remainder_str = next_part;
    }

    Ok(total_duration)
}
pub fn parse_date(input: &str) -> Result<NaiveDate> {
    let now = Local::now().date_naive();
    let trimmed = input.trim();

    match trimmed {
        "t" | "today" => return Ok(now),
        "y" | "yesterday" => return Ok(now - chrono::Duration::days(1)),
        _ => {}
    }

    if let Some(day_str) = trimmed.strip_suffix("-tt") {
        let day: u32 = day_str
            .parse()
            .map_err(|_| eyre!("Invalid day: '{}'", day_str))?;
        return NaiveDate::from_ymd_opt(now.year(), now.month(), day)
            .ok_or_else(|| eyre!("Non-existent date: {}-{}-{}", day, now.month(), now.year()));
    }

    let parts: Vec<&str> = trimmed.split('-').collect();
    if parts.len() == 3 {
        let day: u32 = parts[0]
            .parse()
            .map_err(|_| eyre!("Invalid day: '{}'", parts[0]))?;
        let month: u32 = parts[1]
            .parse()
            .map_err(|_| eyre!("Invalid month: '{}'", parts[1]))?;

        let year: i32 = match parts[2] {
            "t" => now.year(),
            y => y.parse().map_err(|_| eyre!("Invalid year: '{}'", y))?,
        };

        return NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| eyre!("Invalid date: {}-{}-{}", day, month, year));
    }

    NaiveDate::parse_from_str(trimmed, "%-d-%-m-%Y")
        .map_err(|e| eyre!("Failed to parse date'{}': {}", input, e))
}
pub fn save(data: &All) -> Result<()> {
    let path = get_path()?;
    let string_content =
        serde_json::to_string_pretty(data).context("failed to parse data to json")?;
    fs::write(path, &string_content).context("failed to write file")?;
    Ok(())
}
pub fn load_data() -> Result<All> {
    let path = get_path()?;
    if path.exists() {
        let string_conent = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;
        let all_data = serde_json::from_str::<All>(&string_conent)
            .context("failed to parse string to data")?;
        Ok(all_data)
    } else {
        let data = All::default();
        Ok(data)
    }
}

fn get_path() -> Result<PathBuf> {
    let path = dirs::data_local_dir().unwrap().join("time_traker");
    if !path.exists() {
        fs::create_dir_all(&path)
            .with_context(|| format!("Failed to create path: {}", path.display()))?;
    }
    Ok(path.join("data.json"))
}
pub fn get_week(all: &All, week_type: WeekType) -> Vec<(NaiveDate, Day)>{
     match week_type {
        WeekType::SinceToday => {
            let today = Local::now().date_naive();
            let start_week = today - chrono::Duration::days(6);
            all.get_period(start_week, today)
        }
        WeekType::ThisWeek => {
            panic!("this isnt implemented yet")
        }
    }
}
pub fn format_time(time: Duration) -> String{
    let secs = time.as_secs();
    let hours = secs / 3600;
    let mins = (secs % 3600) / 60;
    format!("{:>2}h {:>2}m", hours, mins)
}