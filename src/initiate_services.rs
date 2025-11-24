use std::sync::{Arc, RwLock};

use mime_core::services::{ConfigurationService, implementations::DefaultConfigurationService};
use mime_data::services::{DatasourceService, implementations::PostgresDatasourceService};

use crate::service_container::ServiceContainer;

pub async fn initiate_services() -> anyhow::Result<ServiceContainer> {
    let configuration_service: Arc<RwLock<dyn ConfigurationService>> =
        Arc::new(RwLock::new(DefaultConfigurationService::new(None)));

    mime_core::helpers::set_tracing_subscriber(configuration_service.clone());

    let datasource_service: Arc<RwLock<dyn DatasourceService + Send + Sync>> = Arc::new(RwLock::new(
        PostgresDatasourceService::new(configuration_service.clone()).await,
    ));

    let service_container = ServiceContainer {
        configuration_service,
        datasource_service,
    };

    Ok(service_container)
}
