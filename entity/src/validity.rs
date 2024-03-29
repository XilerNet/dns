//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "validity")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub inscription: String,
    pub address: String,
    #[sea_orm(unique)]
    pub domain: String,
    pub algorithm: String,
    pub public_key: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
