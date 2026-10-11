mod activity;
mod additional_functions;
mod tui;

use crate::activity::{Activity, All};
use crate::additional_functions::format_time;
use chrono::{Local, NaiveDate};
use clap::{Parser, Subcommand, ValueEnum};
use color_eyre::eyre::Result;
use std::collections::BTreeSet;
use std::time::Duration;

#[derive(Parser)]
#[command(author = "lixtrw", version, about = "Time tracker")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Add an activity with tags and a duration
    Add {
        /// Tags describing the activity (e.g. `rust programming`)
        #[arg(value_name = "TAGS", required = true, num_args = 1..)]
        activity_tags: Vec<String>,

        /// Time spent, e.g. `45m`, `1h30m`, `2h`, `90s`
        #[arg(short = 't', long = "time")]
        duration: String,

        /// Date: `t` (today), `y` (yesterday), `12-tt` (day of this month),
        /// `12-01-t` (day-month, this year), `5-10-2026` (day-month-year)
        #[arg(short = 'd', long = "day", default_value = "t")]
        day: String,
    },

    /// Show statistics (today by default)
    Stat {
        #[command(subcommand)]
        stat_type: Option<StatType>,
    },

    /// Remove an activity by its index from `stat`
    Remove {
        /// Index of the activity, as shown by `stat day`
        index: usize,

        /// Date of the activity (same formats as in `add`)
        #[arg(short = 'd', long = "day", default_value = "t")]
        day: String,
    },

    /// Start a TUI stopwatch and save the time when you quit
    ///
    /// Controls: `Space` pauses or resumes, `q` stops and saves.
    StopWatch {
        /// Tags for the activity (e.g. `rust programming`)
        #[arg(required = true)]
        tags: Vec<String>,
    },
}

#[derive(Subcommand)]
enum StatType {
    /// Show all activities for one day
    Day {
        /// Date: `t`, `y`, `12-tt`, `12-01-t`, `5-10-2026`
        #[arg(default_value = "t")]
        day: String,
    },

    /// Show a week summary, day by day
    Week {
        /// Which week to show
        #[arg(value_enum, default_value = "rolling")]
        week_type: WeekType,
    },

    /// Show stats for a custom period (not implemented yet)
    Period {
        /// First day of the period
        since: String,
        /// Last day of the period
        until: String,
    },

    /// Show total time for the given tags
    Tag {
        /// Tags to sum up (matches activities with any of them)
        tags: Vec<String>,
    },
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum WeekType {
    /// The last 7 days, including today
    Rolling,
    /// The current calendar week, starting on Monday
    Current,
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
            add_activity(parsed_day, activity_tags, parsed_duration, &mut data)?;
        }

        Command::Stat { stat_type } => {
            let stat_type = stat_type.unwrap_or(StatType::Day {
                day: "t".to_string(),
            });
            match stat_type {
                StatType::Day { day } => {
                    print_day_stat(&data, &day)?;
                }
                StatType::Week { week_type } => {
                    print_week_stat(&data, week_type)?;
                }
                StatType::Period { since: _, until: _ } => {
                    println!("period is not implemented yet");
                }
                StatType::Tag { tags } => {
                    let a = data.get_tags(&tags);
                    println!(
                        "tags: # {} : {}",
                        tags.to_vec().join(" "),
                        additional_functions::format_time(a)
                    );
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
                    println!(
                        "Activity with index {} not found for date {}",
                        index, parsed_day
                    );
                    return Ok(());
                }
            }
        }
        Command::StopWatch { tags } => {
            let text_tags = tags.join(" ");
            let time = tui::tui::run(&text_tags)?;
            println!("\n{}\n", format_time(time));
            let today = Local::now().date_naive();
            add_activity(today, tags, time, &mut data)?;
        }
    }

    Ok(())
}

fn add_activity(
    day: NaiveDate,
    activity_tags: Vec<String>,
    duration: Duration,
    data: &mut All,
) -> Result<()> {
    let activity = Activity::new(activity_tags, duration);
    data.add_activity(day, activity);

    additional_functions::save(data)?;
    println!("Activity saved at {day}");
    Ok(())
}

fn print_week_stat(data: &All, week_type: WeekType) -> Result<()> {
    let period_data = additional_functions::get_week(data, week_type.clone());

    let Some((first_date, _)) = period_data.first() else {
        println!("No data for this week");
        return Ok(());
    };

    let last_date = period_data.last().unwrap().0;

    println!("Week[{:?}]: {} - {}", week_type, first_date, last_date);
    println!("{:-<55}", "");

    let mut total_week_time = Duration::ZERO;

    for (date, day) in &period_data {
        let total_day_time = day
            .activities
            .iter()
            .map(|activity| activity.time_spent)
            .sum();

        total_week_time += total_day_time;

        let text_time = additional_functions::format_time(total_day_time);

        let activities = day
            .activities
            .iter()
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
    println!(
        "Total: {}",
        additional_functions::format_time(total_week_time)
    );

    Ok(())
}

fn print_day_stat(data: &All, day: &str) -> Result<()> {
    let parsed_day = additional_functions::parse_date(day)?;
    let day_info = match data.get_day(parsed_day) {
        Some(info) => info,
        None => {
            println!("There is nothing for {}", parsed_day);
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
