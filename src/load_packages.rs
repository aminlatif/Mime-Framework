use crate::service_container::ServiceContainer;

pub async fn load_packages(dependency_container: &ServiceContainer) -> anyhow::Result<()> {
    let _ = mime_core::main(dependency_container.configuration_manager.clone()).await;

    let _ = mime_data::main(
        dependency_container.configuration_manager.clone(),
        dependency_container.datasource_service.clone()
    ).await;

    let _ = mime_web::main(dependency_container.configuration_manager.clone()).await;

    Ok(())
}