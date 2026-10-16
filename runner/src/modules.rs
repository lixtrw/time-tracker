use tmtr_plugin_api::Plugin;

pub struct LoadedModules {
    pub modules: Vec<Box<dyn Plugin>>,
}
