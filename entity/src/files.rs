//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "files")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title_bd: String,
    pub title_user: String,
    pub link: String,
    pub date_create: DateTimeWithTimeZone,
    pub date_last_update: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::petitions::Entity")]
    Petitions,
}

impl Related<super::petitions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Petitions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
