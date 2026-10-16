pub use serde;
pub use serde_json;

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
