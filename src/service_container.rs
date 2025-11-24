use std::sync::{Arc, RwLock};

use mime_core::services::ConfigurationService;
use mime_data::services::DatasourceService;

pub struct ServiceContainer {
    pub configuration_service: Arc<RwLock<dyn ConfigurationService>>,
    pub datasource_service: Arc<RwLock<dyn DatasourceService + Send + Sync>>,
}
