use crate::component::Component;

pub mod default_configuration_manager;

pub use default_configuration_manager::DefaultConfigurationManager;

pub trait ConfigurationManager: Component {
    fn save(&self, key: &str, value: &str);
    fn get(&self, key: &str) -> Option<String>;
}