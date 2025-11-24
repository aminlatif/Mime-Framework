use uuid::Uuid;

use crate::entities::user::Model as UserModel;

pub trait UserRepository: Send + Sync {
    async fn get(&self, id: Uuid) -> Result<Option<UserModel>, anyhow::Error>;
    async fn get_all(&self) -> Result<Vec<UserModel>, anyhow::Error>;
    async fn create(&self, model: UserModel) -> Result<UserModel, anyhow::Error>;
    async fn update(&self, model: UserModel) -> Result<UserModel, anyhow::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), anyhow::Error>;
}