use chrono::Utc;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Files::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Files::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Files::TitleBd).string().not_null())
                    .col(ColumnDef::new(Files::TitleUser).string().not_null())
                    .col(ColumnDef::new(Files::Link).string().not_null())
                    .col(
                        ColumnDef::new(Files::DateCreate)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Utc::now().to_string()),
                    )
                    .col(
                        ColumnDef::new(Files::DateLastUpdate)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Utc::now().to_string()),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum Files {
    Table,
    Id,
    TitleBd,
    TitleUser,
    Link,
    DateCreate,
    DateLastUpdate,
}
