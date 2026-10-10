mod activity;
mod additional_functions;

use std::collections::BTreeSet;
use std::time::Duration;
use crate::activity::{Activity, All};
use clap::{Parser, Subcommand};
use color_eyre::eyre::{Result};

#[derive(Parser)]
#[command(author = "lixtr", version, about = "CLI Activity & Time Tracker")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Add {
        #[arg(value_name = "TAGS", required = true, num_args = 1..)]
        activity_tags: Vec<String>,

        #[arg(short = 't', long = "time")]
        duration: String,

        #[arg(short = 'd', long = "day", default_value = "t")]
        day: String,
    },
    Stat {
        #[command(subcommand)]
        stat_type: Option<StatType>
    },
    Remove {
        index: usize,
        #[arg(short = 'd', long = "day", default_value = "t")]
        day: String,
    },
}
#[derive(Subcommand)]
enum StatType{
    Day{
        #[arg(default_value = "t")]
        day: String,
    },
    Week{
        #[command(subcommand)]
        week_type: WeekType
    },
    Period{
        since: String,
        until: String,
    }
}
#[derive(Subcommand)]
pub enum WeekType{
    SinceToday,
    ThisWeek,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let mut data = additional_functions::load_data()?;

    match cli.command {
        Command::Add {
            day,
            activity_tags,
            duration,
        } => {
            let parsed_day = additional_functions::parse_date(&day)?;
            let parsed_duration = additional_functions::format_to_duration(&duration)?;

            let activity = Activity::new(activity_tags, parsed_duration);
            data.add_activity(parsed_day, activity);

            additional_functions::save(&data)?;
            println!("Activity saved at {parsed_day}!");
        }

        Command::Stat { stat_type } => {
            let stat_type = stat_type.unwrap_or(StatType::Day {
                day: "t".to_string(),
            });
            match stat_type {
                StatType::Day { day } => {
                    print_day_stat(&data, &day)?;
                }
                StatType::Week{week_type}=> {
                    print_week_stat(&data,week_type)?;
                }
                StatType::Period { since, until } => {
                    println!("period is not implemented yet");
                }
            }
        }
        Command::Remove { day, index } => {
            let parsed_day = additional_functions::parse_date(&day)?;

            match data.remove_activity_by_index(parsed_day, index) {
                Some(removed) => {
                    additional_functions::save(&data)?;
                    let tags_str = removed.tags.iter().cloned().collect::<Vec<_>>().join(", ");
                    println!("Activity deleted [{}]: #{}", index, tags_str);
                }
                None => {
                    println!("Activity with index {} not found for date {}", index, parsed_day);
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}

fn print_week_stat(data: &All, week_type: WeekType) -> Result<()> {
    let period_data = additional_functions::get_week(data, week_type);

    let Some((first_date, _)) = period_data.first() else {
        println!("No data for this week.");
        return Ok(());
    };

    let last_date = period_data.last().unwrap().0;

    println!("Week: {} - {}", first_date, last_date);
    println!("{:-<55}", "");

    let mut total_week_time = Duration::ZERO;

    for (date, day) in &period_data {
        let total_day_time = day.activities.iter()
            .map(|activity| activity.time_spent).sum();

        total_week_time += total_day_time;

        let text_time = additional_functions::format_time(total_day_time);

        let activities = day.activities.iter()
            .flat_map(|act| act.tags.iter().cloned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .join(", ");

        println!(
            "{:<12} {}  # {}",
            date.format("%a %d.%m"),
            text_time,
            activities
        );
    }

    println!("{:-<55}", "");
    println!("Total: {}", additional_functions::format_time(total_week_time));

    Ok(())
}
fn print_day_stat(data: &All, day: &str) -> Result<()>{
    let parsed_day = additional_functions::parse_date(&day)?;
    let day_info = match data.get_day(parsed_day) {
        Some(info) => info,
        None => {
            println!("There is nothing for {}.", parsed_day);
            return Ok(());
        }
    };
    println!("Statistic for {}:", parsed_day);
    println!("{:-<40}", "");

    let mut total_day_time = Duration::ZERO;

    for (i, act) in day_info.activities.iter().enumerate() {
        total_day_time += act.time_spent;
        let tags_str = act.tags.iter().cloned().collect::<Vec<_>>().join(", ");
        let text_time = additional_functions::format_time(act.time_spent);

        println!("[{}] {}  |  # {}", i, text_time, tags_str);
    }

    println!("{:-<40}", "");
    let total_h = total_day_time.as_secs() / 3600;
    let total_m = (total_day_time.as_secs() % 3600) / 60;
    println!("In total today: {}h {}m", total_h, total_m);
    Ok(())
}
