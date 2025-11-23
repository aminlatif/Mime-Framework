use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("group_authorities")
                    .if_not_exists()
                    .col(ColumnDef::new("group_id").uuid().not_null())
                    .col(ColumnDef::new("authority").string_len(50).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from("group_authorities", "group_id")
                            .to("groups", "id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table("group_authorities").to_owned()).await
    }
}
