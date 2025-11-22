use crate::user_model::User;

pub trait UserService {
    fn get_user(&self, id: u64) -> Option<User>;
    fn create_user(&self, name: &str) -> User;
}
