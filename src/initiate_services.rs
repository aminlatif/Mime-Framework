use std::sync::{Arc, RwLock};

use mime_core::services::{ConfigurationService, implementations::DefaultConfigurationService};
use mime_data::services::{DatasourceService, implementations::PostgresDatasourceService};

use crate::service_container::ServiceContainer;

pub async fn initiate_services() -> anyhow::Result<ServiceContainer> {
    let configuration_service: Arc<RwLock<dyn ConfigurationService>> =
        Arc::new(RwLock::new(DefaultConfigurationService::new(None)));

    mime_core::helpers::set_tracing_subscriber(configuration_service.clone());

    // let connection_string = configuration_manager
    //     .read()
    //     .unwrap()
    //     .get("database_connection_string");

    let datasource_service: Arc<RwLock<dyn DatasourceService>> = Arc::new(RwLock::new(
        PostgresDatasourceService::new(configuration_service.clone()).await,
    ));

    // let migration_service: Arc<RwLock<dyn migration_service::MigrationService>> =
    //     Arc::new(RwLock::new(
    //         migration_service::DefaultMigrationService::new(datasource_service.clone()),
    //     ));

    let service_container = ServiceContainer {
        configuration_service,
        datasource_service,
    };

    Ok(service_container)
}
