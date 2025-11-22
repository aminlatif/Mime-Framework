use std::sync::{Arc, RwLock};

use mime_core::services::ConfigurationService;

use crate::services::WebService;

pub struct DefaultWebService {
    configuration_service: Arc<RwLock<dyn ConfigurationService>>,
}

impl DefaultWebService {
    pub fn new(configuration_service: Arc<RwLock<dyn ConfigurationService>>) -> Self {
        Self {
            configuration_service,
        }
    }
}

impl WebService for DefaultWebService {}
