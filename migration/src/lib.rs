pub use sea_orm_migration::prelude::*;

mod m20240404_231159_users;
mod m20240410_225507_petitions;
mod m20240410_225515_files;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240404_231159_users::Migration),
            Box::new(m20240410_225515_files::Migration),
            Box::new(m20240410_225507_petitions::Migration),
        ]
    }
}
