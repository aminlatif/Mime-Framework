use std::sync::{Arc, Mutex, RwLock};

use once_cell::sync::Lazy;
use sea_orm::DatabaseConnection;
// use sea_orm::{ColIdx, Iden};
pub use sea_orm_migration::prelude::*;
use tracing::{debug, error, info};

use crate::services::{DatasourceService, migration_service::MigrationService};

static MIGRATION_REGISTRY: Lazy<Mutex<Vec<Box<dyn MigrationTrait>>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

pub struct DefaultMigrationService {
    datasource_service: Arc<RwLock<dyn DatasourceService>>,
    migrations: Vec<fn() -> Vec<Box<dyn MigrationTrait>>>,
}

impl DefaultMigrationService {
    pub fn new(
        datasource_service: Arc<RwLock<dyn DatasourceService>>,
        migrations: Vec<fn() -> Vec<Box<dyn MigrationTrait>>>,
    ) -> Self {
        Self {
            datasource_service,
            migrations,
        }
    }
}

impl DefaultMigrationService {
    fn help(&self) {
        println!("Available migration commands:");
        println!("  up <n>      Apply all/<n> pending migrations");
        println!("  down <n>    Rollback all/<n> applied migrations");
        println!("  status      Check the status of all migrations.");
        println!("  fresh       Drop all tables from the database, then reapply all migrations.");
        println!("  refresh <n> Rollback all/<n> applied migrations, then reapply them.");
        println!("  reset       Drop all tables from the database.");
        println!("  help        Show this help message.");
    }

    async fn up(&self, connection: &DatabaseConnection, n: u32) {
        if n != 0 {
            info!("Migration up: Apply {} pending migrations.", n);
            Migrator::up(connection, Some(n)).await.unwrap_or_else(|e| {
                error!("Migration up error: {}", e);
            });
        } else {
            info!("Migration up: Apply all pending migrations.");
            Migrator::up(connection, None).await.unwrap_or_else(|e| {
                error!("Migration up error: {}", e);
            });
        }
    }

    async fn down(&self, connection: &DatabaseConnection, n: u32) {
        if n != 0 {
            info!("Migration down: Rollback {} applied migrations.", n);
            Migrator::down(connection, Some(n))
                .await
                .unwrap_or_else(|e| {
                    error!("Migration down error: {}", e);
                });
        } else {
            info!("Migration down: Rollback all applied migrations.");
            Migrator::down(connection, None).await.unwrap_or_else(|e| {
                error!("Migration down error: {}", e);
            });
        }
    }

    async fn status(&self, connection: &DatabaseConnection) {
        info!("Migration status: Check the status of all migrations.");
        Migrator::status(connection).await.unwrap_or_else(|e| {
            error!("Migration status error: {}", e);
        });
    }

    async fn fresh(&self, connection: &DatabaseConnection) {
        info!("Migration fresh: Drop all tables from the database, then reapply all migrations.");
        Migrator::fresh(connection).await.unwrap_or_else(|e| {
            error!("Migration fresh error: {}", e);
        });
    }

    async fn refresh(&self, connection: &DatabaseConnection, n: u32) {
        info!("Migration refresh: Rollback all applied migrations, then reapply all migrations.");
        self.down(connection, n).await;
        for migration_callback in self.migrations.iter() {
            let migrations = migration_callback();
            for migration in migrations {
                self.register_migration(migration);
            }
        }
        self.up(connection, n).await;
    }

    async fn reset(&self, connection: &DatabaseConnection) {
        info!("Migration reset: Rollback all applied migrations.");
        Migrator::refresh(connection).await.unwrap_or_else(|e| {
            error!("Migration reset error: {}", e);
        });
    }

    pub fn register_migration(&self, migration: Box<dyn MigrationTrait>) {
        MIGRATION_REGISTRY.lock().unwrap().push(migration);
    }
}

impl MigrationService for DefaultMigrationService {
    async fn run_migrations(&self, params: Vec<String>) {
        debug!("Running migrations with arguments: {:#?}", params.join(" "));
        if params.len() > 0 && params.get(0).is_some() {
            let n = params
                .get(1)
                .unwrap_or(&String::from("0"))
                .parse::<u32>()
                .unwrap_or(0);

            for migration_callback in self.migrations.iter() {
                let migrations = migration_callback();
                for migration in migrations {
                    self.register_migration(migration);
                }
            }

            let datasource_service_guard = self.datasource_service.read().unwrap();
            let connection = datasource_service_guard.get_connection();

            match params.get(0).unwrap().trim() {
                "up" => {
                    self.up(connection, n).await;
                }
                "down" => {
                    self.down(connection, n).await;
                }
                "status" => {
                    self.status(connection).await;
                }
                "fresh" => {
                    self.fresh(connection).await;
                }
                "refresh" => {
                    self.refresh(connection, n).await;
                }
                "reset" => {
                    self.reset(connection).await;
                }
                "help" => self.help(),
                _ => {
                    error!("Unknown migration command: {}", params.get(0).unwrap());
                    self.help()
                }
            }
        } else {
            self.help();
            return;
        }
    }
}

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        MIGRATION_REGISTRY.lock().unwrap().drain(..).collect()
    }
}
