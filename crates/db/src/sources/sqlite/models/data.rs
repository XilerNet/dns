// use sea_orm::entity::prelude::*;

// #[derive(DeriveEntityModel, Debug)]
// #[sea_orm(table_name = "domain")]
pub struct DataModel {
    // #[sea_orm(primary_key)]
    pub inscription: String,
    pub domain: String,
    pub data: Vec<u8>,
}
