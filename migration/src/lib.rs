pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20251021_093142_create_user;
mod m20251021_103626_create_user;
mod m20251021_135338_add_user_email;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20251021_093142_create_user::Migration),
            Box::new(m20251021_103626_create_user::Migration),
            Box::new(m20251021_135338_add_user_email::Migration),
        ]
    }
}
