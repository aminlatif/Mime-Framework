use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("acl_object_identity")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("id")
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new("object_id_class").uuid().not_null())
                    .col(ColumnDef::new("object_id_identity").string_len(36).not_null())
                    .col(ColumnDef::new("parent_object").uuid().null())
                    .col(ColumnDef::new("owner_sid").uuid().null())
                    .col(ColumnDef::new("entries_inheriting").boolean().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from("acl_object_identity", "parent_object")
                            .to("acl_object_identity", "id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from("acl_object_identity", "object_id_class")
                            .to("acl_class", "id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from("acl_object_identity", "owner_sid")
                            .to("acl_sid", "id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_acl_object_identity")
                            .table("acl_object_identity")
                            .col("object_id_class")
                            .col("object_id_identity")
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("acl_object_identity").to_owned())
            .await
    }
}
