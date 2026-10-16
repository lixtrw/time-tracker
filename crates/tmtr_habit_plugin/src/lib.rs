use tmtr_api::{load, Plugin, PluginData, store, CommandInfo};
use tmtr_api::serde_json::Value;
use crate::habit::{Habit, TotalHabits};

mod habit;


pub struct HabitPlugin{
    pub meta: PluginData,
}
impl Plugin for HabitPlugin{
    fn meta(&self) -> &PluginData {
        &self.meta
    }
    fn execute(&mut self, command: &str, _args: &[String], data: &mut Value) -> Result<(), String> {
        match command {
            "a" => {
                let mut total_habit : TotalHabits = load(data)?;
                let hab = Habit{name: "math".into(), is_finished: false};
                total_habit.add_habit(hab);
                store(data, &total_habit)?;
                Ok(())

            }
            _ => Err(String::from("Unknown command"))
        }
    }
}

pub fn init() -> Box<dyn Plugin> {
    Box::new(HabitPlugin {
        meta: PluginData {
            id: "habit",
            version: env!("CARGO_PKG_VERSION"),
            commands: vec![CommandInfo {
                name: "add",
                help: "add <tags> < -t time>",
            }],
        },
    })
}
