use tmtr_api::{CommandInfo, Plugin};

struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn id(&self) -> &str {
        "hello"
    }
    fn version(&self) -> &str {
        "0.0.1"
    }
    fn commands(&self) -> Vec<CommandInfo> {
        vec![CommandInfo {
            name: "add",
            help: "",
        }]
    }
    fn execute(&mut self, command: &str, args: &[String], data: &mut Value) -> Result<(), String> {
        match command {
            "add" => {}
            _ => return Err(String::from("Unknown command")),
        }
        Ok(())
    }
}

pub fn init() -> Box<dyn Plugin> {
    Box::new(HelloPlugin)
}

#[cfg(feature = "dynamic")]
#[unsafe(no_mangle)]
fn init_plugin() -> Box<dyn Plugin> {
    init()
}
