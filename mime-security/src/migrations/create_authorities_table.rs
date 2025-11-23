use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("authorities")
                    .if_not_exists()
                    .col(ColumnDef::new("username").string_len(50).not_null())
                    .col(ColumnDef::new("authority").string_len(50).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from("authorities", "username")
                            .to("users", "username")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_auth_username")
                            .table("authorities")
                            .col("username")
                            .col("authority")
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table("authorities").to_owned()).await
    }
}
