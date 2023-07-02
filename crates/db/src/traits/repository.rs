use shared::common::Result;
use xdns_data::models::Domain;

pub trait Repository {
    // TODO: Resolve warning related to self return
    async fn new() -> Self;
    async fn new_memory() -> Self;

    /// Get an existing domain from the repository.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain to get.
    ///
    /// # Returns
    ///
    /// * `Result<Domain>` - The domain if it exists.
    async fn get_domain(&mut self, domain: &str) -> Result<Domain>;

    /// Get an existing domain from the repository by inscription id.
    ///
    /// # Arguments
    ///
    /// * `inscription` - The inscription id of the domain to get.
    ///
    /// # Returns
    ///
    /// * `Result<Domain>` - The domain if it exists.
    async fn get_domain_by_inscription(&mut self, inscription: &str) -> Result<Domain>;

    /// Add a new domain to the repository.
    ///
    /// # Arguments
    ///
    /// * `inscription` - The inscription id of the domain.
    /// * `domain` - The domain to add.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the domain was added.
    async fn add_domain(&mut self, inscription: &str, domain: &Domain) -> bool;

    /// Remove an existing domain from the repository.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain to remove.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the domain was removed.
    async fn remove_domain(&self, domain: &str) -> bool;
}
