use serde_json::Value;
pub use serde_json::*;

pub mod day;

#[derive(Debug)]
pub struct CommandInfo {
    pub name: &'static str,
    pub help: &'static str,
}

pub trait Plugin {
    fn id(&self) -> &str;
    fn version(&self) -> &str;
    fn commands(&self) -> Vec<CommandInfo>;
    fn execute(&mut self, command: &str, args: &[String], data: &mut Value) -> Result<(), String>;
}
