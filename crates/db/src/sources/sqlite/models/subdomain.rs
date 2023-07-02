// use sea_orm::entity::prelude::*;

// #[derive(DeriveEntityModel, Debug)]
// #[sea_orm(table_name = "subdomain")]
pub struct SubdomainModel {
    // #[sea_orm(primary_key)]
    pub inscription: String,
    pub domain: String,
    pub subdomain: String,
    pub rtype: String,
    pub class: String,
    pub ttl: u32,
    pub rdata: String,
}
