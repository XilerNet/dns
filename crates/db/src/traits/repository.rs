use shared::common::Result;
use xdns_data::models::{Data, Domain, SubDomain, Validity, ValidityTransfer};

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

    /// Remove an existing domain from the repository by inscription id.
    ///
    /// # Arguments
    ///
    /// * `inscription` - The inscription id of the domain to remove.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the domain was removed.
    async fn remove_domain_by_inscription(&self, inscription: &str) -> bool;

    /// Add a new subdomain to the repository.
    ///
    /// # Arguments
    ///
    /// * `inscription` - The inscription id of the domain.
    /// * `subdomain` - The subdomain to add.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the subdomain was added.
    async fn add_subdomain(&self, inscription: &str, subdomain: SubDomain) -> bool;

    /// Get all existing subdomains from the repository that match the given domain.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain of the subdomain.
    /// * `subdomain` - The subdomain to get.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<SubDomain>>` - The subdomains if they exist.
    async fn get_subdomain(&self, domain: &str, subdomain: &str) -> Result<Vec<SubDomain>>;

    /// Get an existing subdomain from the repository by inscription id.
    ///
    /// # Arguments
    ///
    /// * `inscription` - The inscription id of the subdomain to get.
    ///
    /// # Returns
    ///
    /// * `Result<SubDomain>` - The subdomain if it exists.
    async fn get_subdomain_by_inscription(&self, inscription: &str) -> Result<SubDomain>;

    /// Remove an existing subdomain from the repository.
    /// This will remove all subdomains that match the given domain and subdomain.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain of the subdomain.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the subdomain was removed.
    async fn remove_subdomains(&self, domain: &str, subdomain: &str) -> bool;

    /// Remove an existing subdomain from the repository by inscription id.
    ///
    /// # Arguments
    ///
    /// * `inscription` - The inscription id of the subdomain to remove.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the subdomain was removed.
    async fn remove_subdomain(&self, inscription: &str) -> bool;

    /// Add a new validity to the repository.
    ///
    /// # Arguments
    ///
    /// * `inscription` - The inscription id of the domain.
    /// * `validity` - The validity to add.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the validity was added.
    async fn add_validity(&self, inscription: &str, validity: Validity) -> bool;

    /// Get a validity from the repository by its domain.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain of the validity.
    ///
    /// # Returns
    ///
    /// * `Result<Validity>` - The validity if it exists.
    async fn get_validity(&self, domain: &str) -> Result<Validity>;

    /// Get a validity from the repository by its inscription id.
    ///
    /// # Arguments
    ///
    /// * `inscription` - The inscription id of the validity.
    ///
    /// # Returns
    ///
    /// * `Result<Validity>` - The validity if it exists.
    async fn get_validity_by_inscription(&self, inscription: &str) -> Result<Validity>;

    /// Remove a validity from the repository by its domain.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain of the validity.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the validity was removed.
    async fn remove_validity(&self, domain: &str) -> bool;

    /// Remove a validity from the repository by its inscription id.
    ///
    /// # Arguments
    ///
    /// * `inscription` - The inscription id of the validity.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the validity was removed.
    async fn remove_validity_by_inscription(&self, inscription: &str) -> bool;

    /// Update a validity in the repository.
    ///
    /// # Arguments
    ///
    /// * `validity` - The validity to update.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the validity was updated.
    async fn update_validity(&self, validity: ValidityTransfer) -> bool;

    /// Update a validity in the repository by its inscription id.
    ///
    /// # Arguments
    ///
    /// * `inscription` - The inscription id of the validity.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the validity was updated.
    async fn update_validity_by_inscription(
        &self,
        inscription: &str,
        validity: ValidityTransfer,
    ) -> bool;

    // TODO: Implement data
    // async fn add_data(&self, inscription: &str, data: Data) -> bool;
    // async fn get_data(&self, domain: &str) -> Result<Vec<Data>>;
    // async fn get_data_by_inscription(&self, inscription: &str) -> Result<Data>;
    // async fn remove_data(&self, inscription: &str) -> bool;
}
