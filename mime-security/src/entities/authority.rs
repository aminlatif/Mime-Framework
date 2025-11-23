use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// The DeriveEntityModel macro does all the heavy lifting of defining an Entity with associating Model, Column and PrimaryKey.
// #[sea_orm::model]
#[derive(DeriveEntityModel, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: Uuid,

    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub username: String,

    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub authority: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::User => Entity::belongs_to(super::user::Entity)
                .from(Column::Username)
                .to(super::user::Column::Username)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
