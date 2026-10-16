use chrono::NaiveDate;
use color_eyre::eyre::Result;
pub use serde;
pub use serde_json;

use crate::day::DaysList;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;

pub mod day;

#[derive(Debug)]
pub struct CommandInfo {
    pub name: &'static str,
    pub help: &'static str,
}

pub struct PluginData {
    pub id: &'static str,
    pub version: &'static str,
    pub commands: Vec<CommandInfo>,
}

pub struct Context<'a> {
    pub date: NaiveDate,
    plugin_id: &'static str,
    day_list: &'a mut DaysList,
}
impl<'a> Context<'a> {
    pub fn data_mut(&mut self) -> &mut Value {
        self.day_list
            .day_mut(self.date)
            .get_data_mut(self.plugin_id)
    }

    pub fn load_range<T: DeserializeOwned + Default>(
        &self,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<(NaiveDate, T)>, String> {
        let raw: Vec<(NaiveDate, &Value)> = self
            .day_list
            .days
            .range(from..=to)
            .filter_map(|(date, day)| day.get_data(self.plugin_id).map(|v| (*date, v)))
            .collect();

        let mut a = Vec::new();
        for (date, v) in raw {
            if let Ok(t) = load::<T>(v) {
                a.push((date, t));
            }
        }

        Ok(a)
    }
}

pub fn load<T: DeserializeOwned + Default>(data: &Value) -> Result<T, String> {
    if data.is_null() {
        return Ok(T::default());
    }
    T::deserialize(data).map_err(|e| e.to_string())
}

pub fn store<T: Serialize>(data: &mut Value, value: &T) -> Result<(), String> {
    *data = serde_json::to_value(value).map_err(|e| e.to_string())?;
    Ok(())
}

pub trait Plugin {
    fn meta(&self) -> &PluginData;

    fn execute(&mut self, command: &str, args: &[String], data: &mut Value) -> Result<(), String>;

    fn id(&self) -> &str {
        self.meta().id
    }
    fn version(&self) -> &str {
        self.meta().version
    }
    fn commands(&self) -> &[CommandInfo] {
        &self.meta().commands
    }
}
