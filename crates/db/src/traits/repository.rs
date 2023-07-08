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
    /// * `Result<(Address, Domain)>` - The domain if it exists.
    async fn get_domain(&self, domain: &str) -> Result<(String, Domain)>;

    /// Get an existing domain from the repository by inscription id.
    ///
    /// # Arguments
    ///
    /// * `inscription` - The inscription id of the domain to get.
    ///
    /// # Returns
    ///
    /// * `Result<(Address, Domain)>` - The domain if it exists.
    async fn get_domain_by_inscription(&self, inscription: &str) -> Result<(String, Domain)>;

    /// Get an existing domain from the repository by address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the domain to get.
    ///
    /// # Returns
    ///
    /// * `Result<Domain>` - The domain if it exists.
    async fn get_domain_by_address(&self, address: &str) -> Result<Domain>;

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
    async fn add_domain(&self, address: &str, inscription: &str, domain: &Domain) -> bool;

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
    async fn add_subdomain(&self, address: &str, inscription: &str, subdomain: SubDomain) -> bool;

    /// Get all existing subdomains from the repository that match the given domain.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain of the subdomain.
    /// * `subdomain` - The subdomain to get.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<(Address, Subdomain)>>` - The subdomains if they exist.
    async fn get_subdomain(
        &self,
        domain: &str,
        subdomain: &str,
    ) -> Result<Vec<(String, SubDomain)>>;

    /// Get an existing subdomain from the repository by inscription id.
    ///
    /// # Arguments
    ///
    /// * `inscription` - The inscription id of the subdomain to get.
    ///
    /// # Returns
    ///
    /// * `Result<(Address, Subdomain)>` - The subdomain if it exists.
    async fn get_subdomain_by_inscription(&self, inscription: &str) -> Result<(String, SubDomain)>;

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
    async fn add_validity(&self, address: &str, inscription: &str, validity: Validity) -> bool;

    /// Get a validity from the repository by its domain.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain of the validity.
    ///
    /// # Returns
    ///
    /// * `Result<(Address, Validity)>` - The validity if it exists.
    async fn get_validity(&self, domain: &str) -> Result<(String, Validity)>;

    /// Get a validity from the repository by its inscription id.
    ///
    /// # Arguments
    ///
    /// * `inscription` - The inscription id of the validity.
    ///
    /// # Returns
    ///
    /// * `Result<(Address, Validity)>` - The validity if it exists.
    async fn get_validity_by_inscription(&self, inscription: &str) -> Result<(String, Validity)>;

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
        address: &str,
        inscription: &str,
        validity: ValidityTransfer,
    ) -> bool;

    /// Add a new data to the repository.
    ///
    /// # Arguments
    ///
    /// * `inscription` - The inscription id of the domain.
    /// * `data` - The data to add.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the data was added.
    async fn add_data(&self, address: &str, inscription: &str, data: Data) -> bool;

    /// Get all existing data from the repository that match the given domain.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain of the data.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<(Address, Data)>>` - The data if they exist. (can be empty)
    async fn get_data(&self, domain: &str) -> Result<Vec<(String, Data)>>;

    /// Get an existing data from the repository by inscription id.
    ///
    /// # Arguments
    ///
    /// * `inscription` - The inscription id of the data to get.
    ///
    /// # Returns
    ///
    /// * `Result<(Address, Data)>` - The data if it exists.
    async fn get_data_by_inscription(&self, inscription: &str) -> Result<(String, Data)>;

    /// Remove an existing data from the repository.
    /// This will remove all data that match the given domain.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain of the data.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the data was removed.
    async fn remove_data(&self, domain: &str) -> bool;

    /// Remove an existing data from the repository by inscription id.
    ///
    /// # Arguments
    ///
    /// * `inscription` - The inscription id of the data to remove.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the data was removed.
    async fn remove_data_by_inscription(&self, inscription: &str) -> bool;
}
