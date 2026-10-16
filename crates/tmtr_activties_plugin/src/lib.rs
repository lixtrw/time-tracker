use tmtr_plugin_api::Plugin;

struct Hello;

impl Plugin for Hello {
    fn id(&self) -> &str {
        "hello"
    }
}

#[unsafe(no_mangle)]
fn init_plugin() -> Box<dyn Plugin> {
    Box::new(Hello)
}
