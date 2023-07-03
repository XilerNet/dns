use log::LevelFilter;
use migration::{IntoCondition, Migrator, MigratorTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ColumnTrait, ConnectOptions, Database, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
};
use std::time::SystemTime;

use entity::subdomain;
use entity::{domain, validity};
use shared::common::Result;
use shared::time::system_time_from_epoch_seconds;
use xdns_data::models::subdomain::{Class as SubDomainClass, Type as SubDomainType};
use xdns_data::models::{Credentials, Domain, SubDomain, Validity, ValidityTransfer};

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
        opt.sqlx_logging(true)
            .sqlx_logging_level(LevelFilter::Debug);
        let connection = Database::connect(opt).await.unwrap();
        Self { connection }
    }

    async fn get_validity_model(&self, domain: &str) -> Result<Option<validity::Model>> {
        Ok(self
            .get_first_entity_by(validity::Entity, validity::Column::Domain.eq(domain))
            .await?)
    }

    fn parse_domain_model(domain_data: Option<domain::Model>) -> Result<Domain> {
        if matches!(domain_data, None) {
            return Err("Domain not found".into());
        }

        let domain_data = domain_data.unwrap();
        let valid_from = domain_data.valid_from.parse::<u64>()?;

        Ok(Domain {
            name: domain_data.name,
            valid_from: system_time_from_epoch_seconds(valid_from),
        })
    }

    fn parse_validity_model(validity_data: Option<validity::Model>) -> Result<Validity> {
        if matches!(validity_data, None) {
            return Err("Validity not found".into());
        }

        let validity = validity_data.unwrap();

        Ok(Validity {
            domain: validity.domain.to_string(),
            credentials: Credentials::new(
                validity.algorithm.try_into()?,
                validity.public_key.to_string(),
            ),
        })
    }

    /// Get the first entity by a filter.
    /// This is a workaround for the lack of a `find_by` method in sea_orm.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to get.
    ///
    /// # Returns
    ///
    /// * `Option<<T as EntityTrait>::Model>` - The entity if it exists.
    async fn get_first_entity_by<T, F>(
        &self,
        _entity: T,
        filter: F,
    ) -> std::result::Result<Option<<T as EntityTrait>::Model>, DbErr>
    where
        T: EntityTrait,
        F: IntoCondition,
    {
        T::find().filter(filter).one(&self.connection).await
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
        let domain_data = self
            .get_first_entity_by(domain::Entity, domain::Column::Name.eq(domain))
            .await?;
        Self::parse_domain_model(domain_data)
    }

    async fn get_domain_by_inscription(&mut self, inscription: &str) -> Result<Domain> {
        let domain_data = self
            .get_first_entity_by(domain::Entity, domain::Column::Inscription.eq(inscription))
            .await?;
        Self::parse_domain_model(domain_data)
    }

    async fn add_domain(&mut self, inscription: &str, domain: &Domain) -> bool {
        let valid_from = domain
            .valid_from
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let domain = domain::ActiveModel {
            name: Set(domain.name.to_string()),
            valid_from: Set(valid_from.to_string()),
            inscription: Set(inscription.to_string()),
        };

        let res = domain::Entity::insert(domain).exec(&self.connection).await;

        matches!(res, Ok(_))
    }

    async fn remove_domain(&self, domain: &str) -> bool {
        let res = domain::Entity::delete_many()
            .filter(domain::Column::Name.eq(domain))
            .exec(&self.connection)
            .await;

        matches!(res, Ok(_)) && res.unwrap().rows_affected != 0
    }

    async fn remove_domain_by_inscription(&self, inscription: &str) -> bool {
        let res = domain::Entity::delete_many()
            .filter(domain::Column::Inscription.eq(inscription))
            .exec(&self.connection)
            .await;

        matches!(res, Ok(_)) && res.unwrap().rows_affected != 0
    }

    async fn add_subdomain(&self, inscription: &str, subdomain: SubDomain) -> bool {
        let subdomain = subdomain::ActiveModel {
            inscription: Set(inscription.to_string()),
            domain: Set(subdomain.domain.to_string()),
            subdomain: Set(subdomain.subdomain.to_string()),
            rtype: Set(subdomain.rtype.to_string()),
            class: Set(subdomain.class.to_string()),
            ttl: Set(subdomain.ttl as i32),
            rdata: Set(subdomain.rdata.to_string()),
        };

        let res = subdomain::Entity::insert(subdomain)
            .exec(&self.connection)
            .await;

        matches!(res, Ok(_))
    }

    async fn get_subdomain(&self, domain: &str, subdomain: &str) -> Result<Vec<SubDomain>> {
        let subdomains = subdomain::Entity::find()
            .filter(
                subdomain::Column::Domain
                    .eq(domain)
                    .and(subdomain::Column::Subdomain.eq(subdomain)),
            )
            .all(&self.connection)
            .await?;

        subdomains
            .iter()
            .map(|domain| {
                Ok(SubDomain {
                    domain: domain.domain.to_string(),
                    subdomain: domain.subdomain.to_string(),
                    rtype: SubDomainType::try_from(&domain.rtype as &str)?,
                    class: SubDomainClass::try_from(&domain.class as &str)?,
                    ttl: domain.ttl as u32,
                    rdata: domain.rdata.to_string(),
                })
            })
            .collect::<Result<Vec<SubDomain>>>()
    }

    async fn get_subdomain_by_inscription(&self, inscription: &str) -> Result<SubDomain> {
        let subdomain_data = self
            .get_first_entity_by(
                subdomain::Entity,
                subdomain::Column::Inscription.eq(inscription),
            )
            .await?;

        if matches!(subdomain_data, None) {
            return Err("Subdomain not found".into());
        }

        let domain = subdomain_data.unwrap();

        // TODO: Refactor to prevent code duplication
        Ok(SubDomain {
            domain: domain.domain.to_string(),
            subdomain: domain.subdomain.to_string(),
            rtype: SubDomainType::try_from(&domain.rtype as &str)?,
            class: SubDomainClass::try_from(&domain.class as &str)?,
            ttl: domain.ttl as u32,
            rdata: domain.rdata.to_string(),
        })
    }

    async fn remove_subdomains(&self, domain: &str, subdomain: &str) -> bool {
        let res = subdomain::Entity::delete_many()
            .filter(
                subdomain::Column::Domain
                    .eq(domain)
                    .and(subdomain::Column::Subdomain.eq(subdomain)),
            )
            .exec(&self.connection)
            .await;

        matches!(res, Ok(_)) && res.unwrap().rows_affected != 0
    }

    async fn remove_subdomain(&self, inscription: &str) -> bool {
        let res = subdomain::Entity::delete_many()
            .filter(subdomain::Column::Inscription.eq(inscription))
            .exec(&self.connection)
            .await;

        matches!(res, Ok(_)) && res.unwrap().rows_affected != 0
    }

    async fn add_validity(&self, inscription: &str, validity: Validity) -> bool {
        let validity = validity::ActiveModel {
            inscription: Set(inscription.to_string()),
            domain: Set(validity.domain.to_string()),
            algorithm: Set(validity.credentials.algorithm.to_string()),
            public_key: Set(validity.credentials.public_key.to_string()),
        };

        let res = validity::Entity::insert(validity)
            .exec(&self.connection)
            .await;

        matches!(res, Ok(_))
    }

    async fn get_validity(&self, domain: &str) -> Result<Validity> {
        Self::parse_validity_model(self.get_validity_model(domain).await?)
    }

    async fn get_validity_by_inscription(&self, inscription: &str) -> Result<Validity> {
        let validity_data = self
            .get_first_entity_by(
                validity::Entity,
                validity::Column::Inscription.eq(inscription),
            )
            .await?;

        Self::parse_validity_model(validity_data)
    }

    async fn remove_validity(&self, domain: &str) -> bool {
        let res = validity::Entity::delete_many()
            .filter(validity::Column::Domain.eq(domain))
            .exec(&self.connection)
            .await;

        matches!(res, Ok(_)) && res.unwrap().rows_affected != 0
    }

    async fn remove_validity_by_inscription(&self, inscription: &str) -> bool {
        let res = validity::Entity::delete_by_id(inscription)
            .exec(&self.connection)
            .await;

        matches!(res, Ok(_)) && res.unwrap().rows_affected != 0
    }

    async fn update_validity(&self, validity: ValidityTransfer) -> bool {
        let current_validity = self.get_validity_model(&validity.domain).await;

        if !matches!(current_validity, Ok(Some(_))) {
            return false;
        }

        let current_validity = current_validity.unwrap().unwrap();
        self.update_validity_by_inscription(&current_validity.inscription, validity)
            .await
    }

    async fn update_validity_by_inscription(
        &self,
        inscription: &str,
        validity: ValidityTransfer,
    ) -> bool {
        if let Some(new_credentials) = validity.new_credentials {
            let raw = validity::ActiveModel {
                inscription: Set(inscription.to_string()),
                domain: Set(validity.domain.to_string()),
                algorithm: Set(new_credentials.algorithm.to_string()),
                public_key: Set(new_credentials.public_key.to_string()),
            };

            let res = validity::Entity::update(raw)
                .filter(validity::Column::Inscription.eq(inscription))
                .exec(&self.connection)
                .await;

            return matches!(res, Ok(_));
        }

        self.remove_validity_by_inscription(inscription).await
    }
}
