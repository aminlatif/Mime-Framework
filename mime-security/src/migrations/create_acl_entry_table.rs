use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("acl_entry")
                    .if_not_exists()
                    .col(ColumnDef::new("id").uuid().not_null().primary_key().default(Expr::cust("gen_random_uuid()")))
                    .col(ColumnDef::new("acl_object_identity").uuid().not_null())
                    .col(ColumnDef::new("ace_order").integer().not_null())
                    .col(ColumnDef::new("sid").uuid().not_null())
                    .col(ColumnDef::new("mask").integer().not_null())
                    .col(ColumnDef::new("granting").boolean().not_null())
                    .col(ColumnDef::new("audit_success").boolean().not_null())
                    .col(ColumnDef::new("audit_failure").boolean().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from("acl_entry", "acl_object_identity")
                            .to("acl_object_identity", "id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from("acl_entry", "sid")
                            .to("acl_sid", "id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_acl_entry_acl_object_identity_ace_order")
                            .table("acl_entry")
                            .col("acl_object_identity")
                            .col("ace_order")
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table("acl_entry").to_owned()).await
    }
}
