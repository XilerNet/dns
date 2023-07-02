use log::LevelFilter;
use migration::{Migrator, MigratorTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ColumnTrait, ConnectOptions, Database, DatabaseConnection, EntityTrait, QueryFilter,
};
use std::time::SystemTime;

use entity::domain;
use shared::common::Result;
use shared::time::system_time_from_epoch_seconds;
use xdns_data::models::Domain;

use crate::traits::Repository;

const FILENAME: &str = "sqlite:./xdns.db?mode=rwc";

pub struct SqliteRepository {
    pub connection: DatabaseConnection,
}

impl SqliteRepository {
    /// Migrate the database to the latest version programmatically.
    /// Ideally used for test purposes.
    pub async fn migrate(&self) {
        Migrator::up(&self.connection, None).await.unwrap();
    }

    async fn make_connection(with: &str) -> Self {
        let mut opt = ConnectOptions::new(with.to_owned());
        opt.sqlx_logging(true).sqlx_logging_level(LevelFilter::Debug);
        let connection = Database::connect(opt).await.unwrap();
        Self { connection }
    }
}

impl Repository for SqliteRepository {
    async fn new() -> Self {
        Self::make_connection(FILENAME).await
    }

    async fn new_memory() -> Self {
        Self::make_connection("sqlite::memory:").await
    }

    async fn get_domain(&mut self, domain: &str) -> Result<Domain> {
        let domain_data = domain::Entity::find().filter(domain::Column::Name.eq(domain)).one(&self.connection).await?;

        if matches!(domain_data, None) {
            return Err("Domain not found".into());
        }

        let domain_data = domain_data.unwrap();
        let valid_from = domain_data.valid_from.parse::<u64>().unwrap();

        Ok(Domain {
            name: domain_data.name,
            valid_from: system_time_from_epoch_seconds(valid_from),
        })
    }

    async fn add_domain(&mut self, inscription: &str, domain: &Domain) -> bool {
        let valid_from = domain.valid_from.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

        let domain = domain::ActiveModel {
            name: Set(domain.name.clone()),
            valid_from: Set(valid_from.to_string()),
            inscription: Set(inscription.to_string()),
        };

        let res = domain::Entity::insert(domain).exec(&self.connection).await;

        matches!(res, Ok(_))
    }

    async fn remove_domain(&self, domain: &str) -> bool {
        let res = domain::Entity::delete_many().filter(domain::Column::Name.eq(domain)).exec(&self.connection).await;

        matches!(res, Ok(_)) && res.unwrap().rows_affected != 0
    }
}
