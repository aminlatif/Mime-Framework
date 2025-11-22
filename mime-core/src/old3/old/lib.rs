use crate::{data_collector::DataCollector, message_service::MessageService};

pub mod data_collector;
pub mod dependency_container;
pub mod simple_data_collector;
pub mod sql_data_collector;
pub mod configuration_manager;
pub mod message_service;
pub mod email_message_service;

pub async fn main() -> anyhow::Result<()> {
    println!("running mime core...");

    let dependency_container = dependency_container::DependencyContainer::new();

    let data_collector_impl = dependency_container.data_collector_impl();
    println!("data_collector_impl: {:#?}", data_collector_impl.collect_data());

    let create_data_collector_dyn = dependency_container.create_data_collector_dyn();
    println!("create_data_collector_dyn: {:#?}", create_data_collector_dyn.collect_data());

    let data_collector_dyn = dependency_container.data_collector_dyn();
    println!("data_collector_dyn: {:#?}", data_collector_dyn.collect_data());

    let configuration_manager = dependency_container.configuration_manager();
    println!("configuration_manager: {:#?}", configuration_manager.get_email_service_username());

    let data_collector = dependency_container.data_collector();
    println!("data_collector: {:#?}", data_collector.collect_data());

    let message_service = dependency_container.message_service();
    message_service.send_message("hi there!");

    Ok(())
}