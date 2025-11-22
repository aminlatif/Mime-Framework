use std::sync::Arc;

use crate::{
    container::Container, notifier_service::NotifierService, user_repository_impl::InMemoryUserRepository, user_service::UserService as _, user_service_impl::DefaultUserService
};

pub mod container;
pub mod notifier_service;
pub mod notifier_service_impl;
pub mod user_model;
pub mod user_repository;
pub mod user_repository_impl;
pub mod user_service;
pub mod user_service_impl;

pub async fn main() -> anyhow::Result<()> {
    let repo = Arc::new(InMemoryUserRepository::new());
    let service = DefaultUserService::new(repo);

    let u = service.create_user("Alice");

    println!("Created: {:?}", u);
    println!("Loaded: {:?}", service.get_user(u.id));

    let mut container = Container::new();

    container.register(NotifierService(Box::new(EmailNotifier)));
    container.register(UserRepoService(Box::new(InMemoryUserRepository)));

    Ok(())
}
