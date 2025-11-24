use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// The DeriveEntityModel macro does all the heavy lifting of defining an Entity with associating Model, Column and PrimaryKey.
#[sea_orm::model]
#[derive(DeriveEntityModel, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub username: String,

    #[sea_orm(column_type = "String(StringLen::N(500))")]
    pub password: String,

    #[sea_orm(column_type = "Boolean")]
    #[serde(skip_deserializing)]
    pub enabled: bool,
}

// #[derive(Copy, Clone, Debug, EnumIter)]
// pub enum Relation {
//     Authority,
// }

// impl RelationTrait for Relation {
//     fn def(&self) -> RelationDef {
//         match self {
//             Relation::User => Entity::has_many(super::authority::Entity)
//                 .from(Column::Username)
//                 .to(super::user::Column::Username)
//                 .into(),
//         }
//     }
// }

impl ActiveModelBehavior for ActiveModel {}
