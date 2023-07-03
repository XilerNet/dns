use shared::common::Result;
use xdns_data::models::Data;

use crate::traits::Repository;

pub trait DataRepository {
    async fn add(&self, inscription: &str, data: Data) -> bool;
    async fn get(&self, domain: &str) -> Result<Vec<Data>>;
    async fn get_by_inscription(&self, inscription: &str) -> Result<Data>;
    async fn remove(&self, domain: &str) -> bool;
    async fn remove_by_inscription(&self, inscription: &str) -> bool;
}

impl<T: Repository> DataRepository for T {
    /// Type specific alias for [`Repository::add_data`].
    async fn add(&self, inscription: &str, data: Data) -> bool {
        self.add_data(inscription, data).await
    }

    /// Type specific alias for [`Repository::get_data`].
    async fn get(&self, domain: &str) -> Result<Vec<Data>> {
        self.get_data(domain).await
    }

    /// Type specific alias for [`Repository::get_data_by_inscription`].
    async fn get_by_inscription(&self, inscription: &str) -> Result<Data> {
        self.get_data_by_inscription(inscription).await
    }

    /// Type specific alias for [`Repository::remove_data`].
    async fn remove(&self, domain: &str) -> bool {
        self.remove_data(domain).await
    }

    /// Type specific alias for [`Repository::remove_data_by_inscription`].
    async fn remove_by_inscription(&self, inscription: &str) -> bool {
        self.remove_data_by_inscription(inscription).await
    }
}
