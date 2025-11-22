use std::sync::Arc;

use crate::{user_model::User, user_repository::UserRepository, user_service::UserService};

pub struct DefaultUserService<R: UserRepository> {
    repo: Arc<R>,
}

impl<R: UserRepository> DefaultUserService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
}

impl<R: UserRepository> UserService for DefaultUserService<R> {
    fn get_user(&self, id: u64) -> Option<User> {
        self.repo.find_by_id(id)
    }

    fn create_user(&self, name: &str) -> User {
        let user = User {
            id: rand::random::<u64>(),
            name: name.to_string(),
        };
        self.repo.save(user.clone());
        user
    }
}
