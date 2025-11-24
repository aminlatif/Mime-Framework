use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("oauth2_authorized_client")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("client_registration_id")
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new("principal_name").string_len(200).not_null())
                    .col(ColumnDef::new("access_token_type").string_len(100).not_null())
                    .col(ColumnDef::new("access_token_value").blob().not_null())
                    .col(
                        ColumnDef::new("access_token_issued_at")
                            .timestamp()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(ColumnDef::new("access_token_expires_at").timestamp().not_null())
                    .col(ColumnDef::new("access_token_scopes").string_len(1000).not_null())
                    .col(ColumnDef::new("refresh_token_value").blob().null())
                    .col(ColumnDef::new("refresh_token_issued_at").timestamp().null())
                    .col(ColumnDef::new("created_at").timestamp().not_null())
                    .primary_key(
                        Index::create()
                            .col("client_registration_id")
                            .col("principal_name")
                            .primary(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("oauth2_authorized_client").to_owned())
            .await
    }
}
