pub use sea_orm_migration::prelude::*;

mod create_acl_class_table;
mod create_acl_entry_table;
mod create_acl_object_identity_table;
mod create_acl_sid_table;
mod create_authorities_table;
mod create_groups_table;
mod create_group_authorities_table;
mod create_group_members_table;
mod create_oauth2_authorized_client_table;
mod create_persistent_logins_table;
mod create_users_table;

pub use create_acl_class_table::Migration as AclClassMigration;
pub use create_acl_entry_table::Migration as AclEntryMigration;
pub use create_acl_object_identity_table::Migration as AclObjectIdentityMigration;
pub use create_acl_sid_table::Migration as AclSidMigration;
pub use create_authorities_table::Migration as AuthorityMigration;
pub use create_groups_table::Migration as GroupMigration;
pub use create_group_authorities_table::Migration as GroupAuthorityMigration;
pub use create_group_members_table::Migration as GroupMemberMigration;
pub use create_oauth2_authorized_client_table::Migration as Oauth2AuthorizedClientMigration;
pub use create_persistent_logins_table::Migration as PersistentLoginMigration;
pub use create_users_table::Migration as UserMigration;

pub fn get_migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
        Box::new(UserMigration),
        Box::new(AuthorityMigration),
        Box::new(GroupMigration),
        Box::new(GroupAuthorityMigration),
        Box::new(GroupMemberMigration),
        Box::new(PersistentLoginMigration),
        Box::new(AclSidMigration),
        Box::new(AclClassMigration),
        Box::new(AclObjectIdentityMigration),
        Box::new(AclEntryMigration),
        Box::new(Oauth2AuthorizedClientMigration),
    ]
}
