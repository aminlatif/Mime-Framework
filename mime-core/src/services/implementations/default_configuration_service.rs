use std::collections::HashMap;

use crate::helpers::read_environment_variables;
use crate::services::ConfigurationService;

pub struct DefaultConfigurationService {
    configurations: HashMap<String, String>,
}

impl DefaultConfigurationService {
    pub fn new(prefix: Option<&str>) -> Self {
        let environemnt_variables = read_environment_variables(prefix);
        let mut configurations = Self {
            configurations: HashMap::new(),
        };

        for (key, value) in environemnt_variables {
            configurations
                .configurations
                .insert(key.to_lowercase(), value);
        }

        configurations
    }
}

impl ConfigurationService for DefaultConfigurationService {
    fn set(&mut self, key: &str, value: &str) {
        self.configurations
            .insert(key.to_lowercase(), value.to_string());
    }

    fn get(&self, key: &str) -> Option<String> {
        self.configurations
            .get(key.to_lowercase().as_str())
            .map(|s| s.to_string())
    }

    fn remove(&mut self, key: &str) {
        self.configurations.remove(key.to_lowercase().as_str());
    }

    fn clear(&mut self) {
        self.configurations.clear();
    }

    fn contains(&self, key: &str) -> bool {
        self.configurations
            .contains_key(key.to_lowercase().as_str())
    }

    fn get_all(&self) -> Vec<(String, String)> {
        self.configurations
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }
}
