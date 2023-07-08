use shared::common::Result;
use xdns_data::models::Domain;

use crate::traits::Repository;

pub trait DomainRepository {
    async fn add(&mut self, address: &str, inscription: &str, domain: &Domain) -> bool;
    async fn get(&mut self, domain: &str) -> Result<(String, Domain)>;
    async fn get_by_inscription(&mut self, inscription: &str) -> Result<(String, Domain)>;
    async fn get_by_address(&mut self, address: &str) -> Result<Domain>;
    async fn remove(&mut self, domain: &str) -> bool;
    async fn remove_by_inscription(&mut self, inscription: &str) -> bool;
}

impl<T: Repository> DomainRepository for T {
    /// Type specific alias for [`Repository::add_domain`].
    async fn add(&mut self, address: &str, inscription: &str, domain: &Domain) -> bool {
        self.add_domain(address, inscription, domain).await
    }

    /// Type specific alias for [`Repository::get_domain`].
    async fn get(&mut self, domain: &str) -> Result<(String, Domain)> {
        self.get_domain(domain).await
    }

    /// Type specific alias for [`Repository::get_domain_by_inscription`].
    async fn get_by_inscription(&mut self, inscription: &str) -> Result<(String, Domain)> {
        self.get_domain_by_inscription(inscription).await
    }

    /// Type specific alias for [`Repository::get_domain_by_address`].
    async fn get_by_address(&mut self, address: &str) -> Result<Domain> {
        self.get_domain_by_address(address).await
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
