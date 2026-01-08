pub use sea_orm_migration::prelude::*;

mod m20220101_000001_init;
mod m20220101_000002_init;
mod m20250701_000001_init;
mod m20250702_023420_user_device;
mod m20250704_085511_feed_back;
mod m20250708_085511_role_group;
mod m202508_07_user_app;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250701_000001_init::Migration),
            Box::new(m20220101_000002_init::Migration),
            Box::new(m20220101_000001_init::Migration),
            Box::new(m20250702_023420_user_device::Migration),
            Box::new(m20250704_085511_feed_back::Migration),
            Box::new(m20250708_085511_role_group::Migration),
            Box::new(m202508_07_user_app::Migration),
        ]
    }
}
