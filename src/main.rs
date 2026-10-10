mod activity;
mod additional_functions;

use crate::activity::{Activity, Day};

use clap::{Parser, Subcommand, builder::Str};
use color_eyre::{Context, Result, eyre::Ok};
use std::time::Duration;

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
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    match cli.command {
        Command::Add {
            day,
            activity_tags,
            duration,
        } => {
            let day = additional_functions::parse_day(&day)?;
            let duration = additional_functions::format_to_duration(&duration)?;
            let activity = Activity::new(activity_tags, duration);
        }
    }
    Ok(())
}
