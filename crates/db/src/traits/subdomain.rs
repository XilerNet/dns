use xdns_data::models::SubDomain;

pub trait SubdomainRepository {
    fn get(&self, domain: &str) -> Vec<SubDomain>;
    fn add(&self, subdomain: &SubDomain) -> bool;
    fn remove(&self, subdomain: &SubDomain) -> bool;
}