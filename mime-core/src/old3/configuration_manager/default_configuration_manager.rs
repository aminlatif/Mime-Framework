use crate::{component::Component, configuration_manager::ConfigurationManager};

pub struct DefaultConfigurationManager;

impl DefaultConfigurationManager {
    pub fn new(container: &crate::container::Container) -> Self {
        Self
    }
}

impl Component for DefaultConfigurationManager {
    // fn new(container: &crate::container::Container) -> Self {
    //     Self
    // }
}

impl ConfigurationManager for DefaultConfigurationManager {
    fn save(&self, key: &str, value: &str) {
        todo!()
    }

    fn get(&self, key: &str) -> Option<String> {
        Some(key.to_string() + " value")
    }
}