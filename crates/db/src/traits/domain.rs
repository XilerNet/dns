use xdns_data::models::Domain;

pub trait DomainRepository {
    fn get(&self, domain: &str) -> Vec<Domain>;
    fn add(&self, domain: &Domain) -> bool;
    fn remove(&self, domain: &Domain) -> bool;
}