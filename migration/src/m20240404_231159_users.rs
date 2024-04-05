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
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::FirstName).string().not_null())
                    .col(ColumnDef::new(Users::LastName).string().not_null())
                    .col(ColumnDef::new(Users::Email).string().not_null())
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .col(ColumnDef::new(Users::Status).string().not_null())
                    .col(ColumnDef::new(Users::Role).boolean().not_null())
                    .col(
                        ColumnDef::new(Users::DateCreate)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Utc::now().to_string()),
                    )
                    .col(
                        ColumnDef::new(Users::DateLastUpdate)
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
enum Users {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    Password,
    Status,
    Role,
    DateCreate,
    DateLastUpdate,
}
