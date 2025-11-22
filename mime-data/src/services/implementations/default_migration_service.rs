use std::sync::{Arc, RwLock};

use crate::services::{DatasourceService, migration_service::MigrationService};

pub struct DefaultMigrationService {
    datasource_service: Arc<RwLock<dyn DatasourceService>>,
}

impl DefaultMigrationService {
    pub fn new(datasource_service: Arc<RwLock<dyn DatasourceService>>) -> Self {
        Self { datasource_service }
    }
}

impl MigrationService for DefaultMigrationService {
    fn run_migration(&self, params: Vec<String>) -> &String {
        todo!()
    }
}
