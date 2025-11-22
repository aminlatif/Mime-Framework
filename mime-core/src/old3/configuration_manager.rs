pub mod default_configuration_manager;

pub use default_configuration_manager::DefaultConfigurationManager;

pub trait ConfigurationManager {
    fn set(&mut self, key: &str, value: &str);

    fn get(&self, key: &str) -> Option<String>;

    fn remove(&mut self, key: &str);

    fn clear(&mut self);

    fn contains(&self, key: &str) -> bool;

    fn get_all(&self) -> Vec<(String, String)>;
}
