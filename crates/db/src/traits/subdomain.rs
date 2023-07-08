use crate::traits::Repository;
use shared::common::Result;
use xdns_data::models::SubDomain;

pub trait SubdomainRepository {
    async fn add(&self, address: &str, inscription: &str, subdomain: SubDomain) -> bool;
    async fn get(&self, domain: &str, subdomain: &str) -> Result<Vec<(String, SubDomain)>>;
    async fn get_by_inscription(&self, inscription: &str) -> Result<(String, SubDomain)>;
    async fn remove_all(&self, domain: &str, subdomain: &str) -> bool;
    async fn remove(&self, inscription: &str) -> bool;
}

impl<T: Repository> SubdomainRepository for T {
    /// Type specific alias for [`Repository::add_subdomain`].
    async fn add(&self, address: &str, inscription: &str, subdomain: SubDomain) -> bool {
        self.add_subdomain(address, inscription, subdomain).await
    }

    /// Type specific alias for [`Repository::get_subdomain`].
    async fn get(&self, domain: &str, subdomain: &str) -> Result<Vec<(String, SubDomain)>> {
        self.get_subdomain(domain, subdomain).await
    }

    /// Type specific alias for [`Repository::get_subdomain_by_inscription`].
    async fn get_by_inscription(&self, inscription: &str) -> Result<(String, SubDomain)> {
        self.get_subdomain_by_inscription(inscription).await
    }

    /// Type specific alias for [`Repository::remove_subdomains`].
    async fn remove_all(&self, domain: &str, subdomain: &str) -> bool {
        self.remove_subdomains(domain, subdomain).await
    }

    /// Type specific alias for [`Repository::remove_subdomain`].
    async fn remove(&self, inscription: &str) -> bool {
        self.remove_subdomain(inscription).await
    }
}
