use crate::user_model::User;

pub trait UserRepository: Send + Sync {
    fn find_by_id(&self, id: u64) -> Option<User>;
    fn save(&self, user: User);
}
