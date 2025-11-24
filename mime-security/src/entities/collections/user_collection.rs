use crate::entities::user::Model as UserModel;

pub trait UserCollection: Send + Sync {
    async fn get_users(&self) -> Result<Vec::<UserModel>, anyhow::Error>;
}