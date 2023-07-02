use shared::common::Result;
use xdns_data::models::Domain;

use crate::traits::Repository;

pub trait DomainRepository {
    async fn get(&mut self, domain: &str) -> Result<Domain>;
    async fn add(&mut self, inscription: &str, domain: &Domain) -> bool;
}

impl<T: Repository> DomainRepository for T {
    async fn get(&mut self, domain: &str) -> Result<Domain> {
        self.get_domain(domain).await
    }

    async fn add(&mut self, inscription: &str, domain: &Domain) -> bool {
        self.add_domain(inscription, domain).await
    }
}
