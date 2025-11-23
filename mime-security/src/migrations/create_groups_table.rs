use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("groups")
                    .if_not_exists()
                    .col(ColumnDef::new("id").uuid().not_null().primary_key().default(Expr::cust("gen_random_uuid()")))
                    .col(ColumnDef::new("group_name").string_len(50).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table("groups").to_owned()).await
    }
}
