use std::sync::{Arc, RwLock};

use tracing::info;

use crate::configuration_manager::ConfigurationManager;

pub fn set_tracing_subscriber(configuration_manager: Arc<RwLock<dyn ConfigurationManager>>) {
    let trace_level_string = configuration_manager.read().unwrap().get("trace_level").unwrap_or("ERROR".to_string());

    let trace_level = match trace_level_string.to_lowercase().as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => panic!("\x1b[31mInvalid trace level: {}. Check TRACE_LEVEL environment variable. Exiting now...\x1b[0m", trace_level_string),
    };

    tracing_subscriber::fmt()
        .with_max_level(trace_level)
        .with_target(false)
        .init();

    info!("Trace level: {}", trace_level);
}
