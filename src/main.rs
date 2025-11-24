use mime_web::{
    services::{WebService, implementations::DefaultWebService},
    types::{AppState, RouteItems, TemplatePath},
};
use sea_orm_migration::MigrationTrait;
use std::env;
use tracing::info;

use mime_data::services::{MigrationService, implementations::DefaultMigrationService};

mod initiate_services;
mod service_container;

use initiate_services::initiate_services;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let dependency_container = initiate_services().await?;

    let args: Vec<String> = env::args().collect();

    let argument_1 = args.get(1).unwrap_or(&String::from("None")).to_owned();
    let argument_2 = args.get(2).unwrap_or(&String::from("None")).to_owned();
    let argument_3 = args.get(3).unwrap_or(&String::from("None")).to_owned();

    match argument_1.trim() {
        "migrate" => {
            let migrations: Vec<fn() -> Vec<Box<dyn MigrationTrait>>> = vec![mime_security::migrations::get_migrations];
            let migration_service =
                DefaultMigrationService::new(dependency_container.datasource_service.clone(), migrations);
            migration_service.run_migrations(vec![argument_2, argument_3]).await;
        }
        _ => {
            info!("Getting ready to start core application...");
            let router_items_list: Vec<RouteItems<AppState>> =
                vec![mime_web::routes::get_routes(), mime_security::routes::get_routes()];
            let tempaltes_list_list: Vec<Vec<TemplatePath>> =
                vec![mime_web::view::get_templates(), mime_security::view::get_templates()];
            let mut web_service = DefaultWebService::new(
                dependency_container.configuration_service.clone(),
                dependency_container.datasource_service.clone(),
                router_items_list,
                tempaltes_list_list,
            );
            web_service.start().await?;
        }
    }

    // mime_core::start(dependency_container.configuration_manager.clone()).await?;

    // load_packages(&dependency_container).await?;

    Ok(())
}
