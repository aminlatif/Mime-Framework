pub mod helpers;

pub mod services;

// use std::sync::{Arc, RwLock};
// use tracing::info;

// pub use configuration_manager::{ConfigurationManager, DefaultConfigurationManager};

// pub mod utils;
// pub mod collection;
// pub mod configuration_manager;
// pub mod repository;

// pub async fn start(configuration_manager: Arc<RwLock<dyn ConfigurationManager>>) -> anyhow::Result<()> {
//     {
//         let mut write_guard = configuration_manager.write().unwrap();
//         write_guard.set("core_start_date_time", chrono::Utc::now().to_rfc3339().as_str());
//     }
//     utils::set_tracing_subscriber(configuration_manager.clone());
//     info!("Configurations: {:#?}", configuration_manager.read().unwrap().get_all());

//     Ok(())
// }
