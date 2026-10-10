mod activity;
mod additional_functions;

use crate::activity::Activity;

use clap::{Parser, Subcommand};
use color_eyre::{
    Result,
    eyre::{ContextCompat, Ok},
};

#[derive(Parser)]
#[command(author = "lixtr", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Add {
        day: String,
        #[arg(value_name = "TAGS", num_args = 1..)]
        activity_tags: Vec<String>,
        #[arg(short, long)]
        duration: String,
    },
    Stat {
        day: String,
    },
    Remove {
        day: String,
        index_activity: usize,
    },
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
            let day = additional_functions::parse_day(&day)?;
            let duration = additional_functions::format_to_duration(&duration)?;

            let activity = Activity::new(activity_tags, duration);
            data.add_activity(day, activity);
        }
        Command::Stat { day } => {
            let day = additional_functions::parse_day(&day)?;
            let stat = data.get_day(day).context("no day")?;
            println!("{:#?}", stat);
        }
        Command::Remove {
            day,
            index_activity,
        } => {
            let day = additional_functions::parse_day(&day)?;
            data.remove_activity_by_index(day, index_activity);
        }
    }
    additional_functions::save(&data)?;
    Ok(())
}
