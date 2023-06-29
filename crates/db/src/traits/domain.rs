use xdns_data::models::Domain;
use crate::Repository;

pub trait DomainRepository {
    fn get(&self, domain: &str) -> Vec<Domain>;
    fn add(&self, domain: &Domain) -> bool;
    fn remove(&self, domain: &Domain) -> bool;
}

impl<T: Repository> DomainRepository for T {
    fn get(&self, domain: &str) -> Vec<Domain> {
        self.get_domain(domain)
    }

    fn add(&self, domain: &Domain) -> bool {
        self.add_domain(domain)
    }

    fn remove(&self, domain: &Domain) -> bool {
        self.remove_domain(domain)
    }
}