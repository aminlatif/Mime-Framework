pub mod services;

// use std::sync::{Arc, RwLock};

// pub use datasource_service::{DatasourceService, PostgresDatasourceService};
// pub use migration_service::{MigrationService, DefaultMigrationService};
// use mime_core::ConfigurationManager;
// use tracing::info;

// pub mod datasource_service;
// pub mod migration_service;

// pub async fn main(
//     configuration_manager: Arc<RwLock<dyn ConfigurationManager>>,
//     datasource_service: Arc<RwLock<dyn DatasourceService>>
// ) -> anyhow::Result<()> {
//     info!("running mime data...");

//     Ok(())
// }
