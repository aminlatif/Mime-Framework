use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::user_model::User;
use crate::user_repository::UserRepository;

pub struct InMemoryUserRepository {
    store: Mutex<HashMap<u64, User>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    fn find_by_id(&self, id: u64) -> Option<User> {
        let store = self.store.lock().unwrap();
        store.get(&id).cloned()
    }

    fn save(&self, user: User) {
        let mut store = self.store.lock().unwrap();
        store.insert(user.id, user);
    }
}
