// use std::sync::Arc;

use std::sync::{Arc, RwLock};

use mime_data::services::DatasourceService;
use sea_orm::DatabaseConnection;
use tera::Tera;

// use crate::{core::Repository, modules::user::{UserActiveModel, UserModel}};

#[derive(Clone)]
pub struct AppState {
    // pub user_repository: Arc<Repository<UserModel, UserActiveModel>>,
    pub templates: Tera,
    pub database_connection: DatabaseConnection,
}