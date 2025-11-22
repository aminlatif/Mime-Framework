use std::sync::{Arc, RwLock};
use tracing::debug;

use mime_core::services::ConfigurationService;

use crate::services::DatasourceService;

pub struct PostgresDatasourceService {
    connection_string: String,
    connection_options: sea_orm::ConnectOptions,
    connection: sea_orm::DatabaseConnection,
}

impl PostgresDatasourceService {
    pub async fn new(configuration_manager: Arc<RwLock<dyn ConfigurationService>>) -> Self {
        // let database_conection_string: String;

        let connection_string = configuration_manager.read().unwrap().get("database_connection_string").unwrap_or_else(|| {
            panic!("\x1b[31mFatal error while reading \"DATABASE_CONNECTION_STRING\".\nNo \"DATABASE_CONNECTION_STRING\", no application.\nExiting.\x1b[0m");
        });
        // if connection_string.is_none() {

        // }
        // if connection_string.is_none() {
        //     debug!(
        //         "No connection string provided. Trying to read DATABASE_CONNECTION_STRING environment variable..."
        //     );
        //     database_conection_string = std::env::var("DATABASE_CONNECTION_STRING").unwrap_or_else(|e| {
        //         panic!("\x1b[31mFatal error while reading \"DATABASE_CONNECTION_STRING\": {}.\nNo \"DATABASE_CONNECTION_STRING\", no application.\nExiting.\x1b[0m", e);
        //     });
        // } else {
        //     database_conection_string = connection_string.unwrap();
        // }

        debug!("* Database connection string ready.");

        let connection_options = sea_orm::ConnectOptions::new(&connection_string);

        debug!("* Database connection options ready.");

        let connection = sea_orm::Database::connect(connection_options.clone())
            .await
            .unwrap_or_else(|e| {
                panic!(
                    "\x1b[31m{}\nNo database, no application.\nExiting.\x1b[0m",
                    e
                );
            });

        debug!("* Database connection ready.");

        Self {
            connection_string: connection_string,
            connection_options: connection_options,
            connection: connection,
        }
    }
}

impl DatasourceService for PostgresDatasourceService {
    fn get_connection_string(&self) -> &String {
        &self.connection_string
    }

    fn get_connection_options(&self) -> &sea_orm::ConnectOptions {
        &self.connection_options
    }

    fn get_connection(&self) -> &sea_orm::DatabaseConnection {
        &self.connection
    }
}
