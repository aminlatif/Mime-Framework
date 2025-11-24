use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("acl_sid")
                    .if_not_exists()
                    .col(ColumnDef::new("id").uuid().not_null().primary_key().default(Expr::cust("gen_random_uuid()")))
                    .col(ColumnDef::new("principal").boolean().not_null())
                    .col(ColumnDef::new("sid").string_len(100).not_null())
                    .index(
                        Index::create()
                            .name("idx_acl_sid_sid_principal")
                            .table("acl_sid")
                            .col("sid")
                            .col("principal")
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table("acl_sid").to_owned()).await
    }
}
