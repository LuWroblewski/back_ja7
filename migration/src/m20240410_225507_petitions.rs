use chrono::Utc;
use sea_orm_migration::prelude::*;

use crate::{m20240404_231159_users::Users, m20240410_225515_files::Files};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Petitions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Petitions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Petitions::Description).string().not_null())
                    .col(ColumnDef::new(Petitions::Title).string().not_null())
                    .col(ColumnDef::new(Petitions::Theme).string().not_null())
                    .col(ColumnDef::new(Petitions::TypePetition).string().not_null())
                    .col(ColumnDef::new(Petitions::Status).string().not_null())
                    .col(ColumnDef::new(Petitions::FileId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Petitions::Table, Petitions::FileId)
                            .to(Files::Table, Files::Id),
                    )
                    .col(
                        ColumnDef::new(Petitions::DateCreate)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Utc::now().to_string()),
                    )
                    .col(
                        ColumnDef::new(Petitions::UserLastUpdate)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Petitions::Table, Petitions::UserLastUpdate)
                            .to(Users::Table, Users::Id),
                    )
                    .col(
                        ColumnDef::new(Petitions::DateLastUpdate)
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
enum Petitions {
    Table,
    Id,
    Description,
    Title,
    Theme,
    TypePetition,
    Status,
    FileId,
    DateCreate,
    UserLastUpdate,
    DateLastUpdate,
}
