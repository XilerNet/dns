// use sea_orm::entity::prelude::*;

// #[derive(DeriveEntityModel, Debug)]
// #[sea_orm(table_name = "validity")]
pub struct ValidityModel {
    // #[sea_orm(primary_key)]
    pub inscription: String,
    pub domain: String,
    pub algorithm: String,
    pub public_key: String,
}
