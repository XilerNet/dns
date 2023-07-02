use shared::common::Result;
use xdns_data::models::Domain;

use crate::traits::Repository;

pub trait DomainRepository {
    async fn get(&mut self, domain: &str) -> Result<Domain>;
    async fn get_by_inscription(&mut self, inscription: &str) -> Result<Domain>;
    async fn add(&mut self, inscription: &str, domain: &Domain) -> bool;
    async fn remove(&mut self, domain: &str) -> bool;
    async fn remove_by_inscription(&mut self, inscription: &str) -> bool;
}

impl<T: Repository> DomainRepository for T {
    /// Type specific alias for [`Repository::get_domain`].
    async fn get(&mut self, domain: &str) -> Result<Domain> {
        self.get_domain(domain).await
    }

    /// Type specific alias for [`Repository::get_domain_by_inscription`].
    async fn get_by_inscription(&mut self, inscription: &str) -> Result<Domain> {
        self.get_domain_by_inscription(inscription).await
    }

    /// Type specific alias for [`Repository::add_domain`].
    async fn add(&mut self, inscription: &str, domain: &Domain) -> bool {
        self.add_domain(inscription, domain).await
    }

    /// Type specific alias for [`Repository::remove_domain`].
    async fn remove(&mut self, domain: &str) -> bool {
        self.remove_domain(domain).await
    }

    /// Type specific alias for [`Repository::remove_domain_by_inscription`].
    async fn remove_by_inscription(&mut self, inscription: &str) -> bool {
        self.remove_domain_by_inscription(inscription).await
    }
}
