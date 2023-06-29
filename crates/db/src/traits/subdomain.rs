use xdns_data::models::SubDomain;
use crate::Repository;

pub trait SubdomainRepository {
    fn get(&self, domain: &str) -> Vec<SubDomain>;
    fn add(&self, subdomain: &SubDomain) -> bool;
    fn remove(&self, subdomain: &SubDomain) -> bool;
}

impl<T: Repository> SubdomainRepository for T {
    fn get(&self, domain: &str) -> Vec<SubDomain> {
        self.get_subdomain(domain)
    }

    fn add(&self, subdomain: &SubDomain) -> bool {
        self.add_subdomain(subdomain)
    }

    fn remove(&self, subdomain: &SubDomain) -> bool {
        self.remove_subdomain(subdomain)
    }
}