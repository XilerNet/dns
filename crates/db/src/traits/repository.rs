use shared::common::Result;
use xdns_data::models::Domain;

pub trait Repository {
    // TODO: Resolve warning related to self return
    async fn new() -> Self;
    async fn new_memory() -> Self;

    async fn get_domain(&mut self, domain: &str) -> Result<Domain>;
    async fn add_domain(&mut self, inscription: &str, domain: &Domain) -> bool;
    async fn remove_domain(&self, domain: &str) -> bool;
}
