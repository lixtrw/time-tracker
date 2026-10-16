use anyhow::Context;
use anyhow::Result;
use std::time::Duration;
use tmtr_api::serde_json::Value;
use tmtr_api::{CommandInfo, Plugin, PluginData};
use tmtr_api::{load, store};

mod activity;

use crate::activity::*;

struct ActivityPlugin {
    pub meta: PluginData,
}
impl Plugin for ActivityPlugin {
    fn meta(&self) -> &PluginData {
        &self.meta
    }

    fn execute(&mut self, command: &str, args: &[String], data: &mut Value) -> Result<(), String> {
        match command {
            "add" => {
                let parced = parse_args(args).map_err(|e| e.to_string())?;
                let activity = Activity::new(parced.1, parced.0);

                let mut total: TotalActivities = load(data)?;
                total.add_activity(activity);
                store(data, &total)?;

                Ok(())
            }
            _ => Err(format!("Unknown command: {command}")),
        }
    }
}

fn parse_args(args: &[String]) -> Result<(Duration, Vec<String>)> {
    let arg_pos = args.iter().position(|a| a == "-t").context("no time")?;
    let time = args[arg_pos + 1]
        .parse::<u64>()
        .context("time should be in mins")?;
    let tags: Vec<String> = args[0..arg_pos].into();

    Ok((Duration::from_mins(time), tags))
}

pub fn init() -> Box<dyn Plugin> {
    Box::new(ActivityPlugin {
        meta: PluginData {
            id: "activity",
            version: env!("CARGO_PKG_VERSION"),
            commands: vec![CommandInfo {
                name: "add",
                help: "add <tags> < -t time>",
            }],
        },
    })
}

#[cfg(feature = "dynamic")]
#[unsafe(no_mangle)]
fn init_plugin() -> Box<dyn Plugin> {
    init()
}
