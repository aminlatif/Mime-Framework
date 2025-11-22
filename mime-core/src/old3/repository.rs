// use std::default;

// use sea_orm::{DatabaseConnection, DbErr, DeriveEntityModel, EntityTrait, sea_query::Mode};
// use serde::de;
// use uuid::Uuid;

// use super::collection::Collection;

// pub struct Repository<ModelType, ActiveModelType> { //where ModelType: EntityTrait {
//     database_connection: DatabaseConnection,

//     read_callback: fn(&Self, id: Option<Uuid>) -> Result<Option<Collection<ModelType>>, DbErr>,

//     create_callback: fn(&Self, active_model: ActiveModelType) -> Result<ModelType, DbErr>,

//     update_callback: fn(&Self, id: Uuid, active_model: ActiveModelType) -> Result<ModelType, DbErr>,

//     delete_callback: fn(&Self, id: Uuid) -> Result<ModelType, DbErr>,
// }

// impl<ModelType, ActiveModelType> Default for Repository<ModelType, ActiveModelType> {
//     fn default() -> Repository<ModelType, ActiveModelType> {
//         let default_read_callback =
//             |repository: &Self, id: Option<Uuid>| -> Result<Option<Collection<ModelType>>, DbErr> {
//                 println!("respository get");
//                 todo!()
//             };

//         let default_create_callback =
//             |repository: &Self, active_model: ActiveModelType| -> Result<ModelType, DbErr> {
//                 println!("respository create");
//                 todo!()
//             };

//         let default_update_callback =
//             |repository: &Self, id: Uuid, active_model: ActiveModelType| -> Result<ModelType, DbErr> {
//                 println!("respository update");
//                 todo!()
//             };

//         let default_delete_callback =
//             |repository: &Self, id: Uuid| -> Result<ModelType, DbErr> {
//                 println!("respository delete");
//                 todo!()
//             };

//         Repository {
//             database_connection: Default::default(),
//             read_callback: default_read_callback,
//             create_callback: default_create_callback,
//             update_callback: default_update_callback,
//             delete_callback: default_delete_callback,
//         }
//     }
// }

// impl<ModelType, ActiveModelType> Repository<ModelType, ActiveModelType> {
//     pub fn new(database_connection: DatabaseConnection) -> Self
//     where
//         Self: Sized,
//     {
//         Self {
//             database_connection,
//             ..Self::default() // read_callback: |_, _| todo!(),
//                               // create_callback: |_, _| todo!(),
//                               // update_callback: |_, _, _| todo!(),
//                               // delete_callback: |_, _| todo!(),
//         }
//     }

//     fn get_db_connection(&self) -> &DatabaseConnection {
//         &self.database_connection
//     }

//     pub async fn get(&self, id: Option<Uuid>) -> Result<Option<Collection<ModelType>>, DbErr> {
//         (self.read_callback)(self, id)
//     }

//     pub async fn create(&self, active_model: ActiveModelType) -> Result<ModelType, DbErr> {
//         (self.create_callback)(self, active_model)
//     }

//     pub async fn update(
//         &self,
//         id: Uuid,
//         active_model: ActiveModelType,
//     ) -> Result<ModelType, DbErr> {
//         (self.update_callback)(self, id, active_model)
//     }

//     pub async fn delete(&self, id: Uuid) -> Result<ModelType, DbErr> {
//         (self.delete_callback)(self, id)
//     }
// }
