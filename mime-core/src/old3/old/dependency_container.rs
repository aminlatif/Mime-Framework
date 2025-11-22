use chrono::{DateTime, Utc};
use std::cell::OnceCell;

use super::email_message_service::EmailMessageService;

use super::message_service::MessageService;

use super::configuration_manager::ConfigurationManager;

use super::sql_data_collector::SqlDataCollector;

use super::simple_data_collector::SimpleDataCollector;

use super::data_collector::DataCollector;

// The container we are going to use to resolve dependencies.
// Like `ServiceCollection` in .NET Core
// And `ApplicationContext` in Spring
pub struct DependencyContainer {
    // Singleton
    configuration_manager: OnceCell<ConfigurationManager>,
}

impl DependencyContainer {
    pub fn new() -> Self {
        Self {
            configuration_manager: OnceCell::new(),
        }
    }

    // We just make a function to return the concrete type
    pub fn datetime(&self) -> DateTime<Utc> {
        Utc::now()
    }

    // This time we are returning an abstract type
    // This allows us to change the implementation of this function to change
    // the low-level details without changing the business code
    pub fn data_collector_impl(&self) -> impl DataCollector {
        SimpleDataCollector::new("api_key".to_string())
    }

    // Attempt to conditional choose the [DataCollector] at runtime
    // error[E0308]: `if` and `else` have incompatible types
    // pub fn create_data_collector_impl_error(&self) -> impl DataCollector {
    //     if false {
    //         SimpleDataCollector::new("api_key".to_string())
    //     } else {
    //         SqlDataCollector::new("connection_string".to_string())
    //     }
    // }

    // error[E0746]: return type cannot be a trait object without pointer indirection
    // doesn't have a size known at compile-time
    // pub fn create_data_collector_dyn_error(&self) -> dyn DataCollector {
    //     if false {
    //         SimpleDataCollector::new("api_key".to_string())
    //     } else {
    //         SqlDataCollector::new("connection_string".to_string())
    //     }
    // }

    // We can now conditionally choose which [DataCollector] to use at runtime
    pub fn create_data_collector_dyn(&self) -> Box<dyn DataCollector> {
        if false {
            Box::new(SimpleDataCollector::new("api_key".to_string()))
        } else {
            Box::new(SqlDataCollector::new("connection_string".to_string()))
        }
    }

    // Things calling this method does not care that it might be a boxed type
    // They just want a [DataCollector]
    pub fn data_collector_dyn(&self) -> impl DataCollector {
        self.create_data_collector_dyn()
    }

    fn create_configuration_manager(&self) -> ConfigurationManager {
        ConfigurationManager::new()
    }

    // pub fn configuration_manager(&self) -> ConfigurationManager {
    //     self.create_configuration_manager()
    // }

    pub fn configuration_manager(&self) -> &ConfigurationManager {
        // Use [OnceCell] to only create the [ConfigurationManager] once
        self.configuration_manager
            .get_or_init(|| self.create_configuration_manager())
    }

    // Create [DataCollector] based on the configuration
    fn create_data_collector(
        &self,
        configuration_manager: &ConfigurationManager,
    ) -> Box<dyn DataCollector> {
        if let Some(api_key) = configuration_manager.get_api_key() {
            Box::new(SimpleDataCollector::new(api_key.to_string()))
        } else {
            Box::new(SqlDataCollector::new(
                configuration_manager
                    .get_database_connection_string()
                    .expect("api key or connection string to be set")
                    .to_string(),
            ))
        }
    }

    // Developers can call this method easily without needing to know about the
    // dependency on [ConfigurationManager]
    pub fn data_collector(&self) -> impl DataCollector {
        let configuration_manager = self.configuration_manager();
        self.create_data_collector(configuration_manager)
    }

    // Get a [MessageService] which also needs the [ConfigurationManager]
    fn create_message_service(
        &self,
        configuration_manager: &ConfigurationManager,
    ) -> impl MessageService {
        EmailMessageService::new(
            configuration_manager
                .get_email_service_username()
                .to_string(),
            configuration_manager
                .get_email_service_password()
                .to_string(),
        )
    }

    pub fn message_service(&self) -> impl MessageService {
        self.create_message_service(self.configuration_manager())
    }
}
