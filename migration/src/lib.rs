pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_domain_table;
mod m20230702_073858_create_subdomain_table;
mod m20230702_081359_create_validity_table;
mod m20230702_081403_create_data_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_domain_table::Migration),
            Box::new(m20230702_073858_create_subdomain_table::Migration),
            Box::new(m20230702_081359_create_validity_table::Migration),
            Box::new(m20230702_081403_create_data_table::Migration),
        ]
    }
}
