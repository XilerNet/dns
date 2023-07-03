use shared::common::Result;
use xdns_data::models::{Validity, ValidityTransfer};

use crate::traits::Repository;

pub trait ValidityRepository {
    async fn add(&self, inscription: &str, validity: Validity) -> bool;
    async fn get(&self, domain: &str) -> Result<Validity>;
    async fn get_by_inscription(&self, inscription: &str) -> Result<Validity>;
    async fn remove(&self, domain: &str) -> bool;
    async fn remove_by_inscription(&self, inscription: &str) -> bool;
    async fn update(&self, validity: ValidityTransfer) -> bool;
    async fn update_by_inscription(&self, inscription: &str, validity: ValidityTransfer) -> bool;
}

impl<T: Repository> ValidityRepository for T {
    /// Type specific alias for [`Repository::add_validity`].
    async fn add(&self, inscription: &str, validity: Validity) -> bool {
        self.add_validity(inscription, validity).await
    }

    /// Type specific alias for [`Repository::get_validity`].
    async fn get(&self, domain: &str) -> Result<Validity> {
        self.get_validity(domain).await
    }

    /// Type specific alias for [`Repository::get_validity_by_inscription`].
    async fn get_by_inscription(&self, inscription: &str) -> Result<Validity> {
        self.get_validity_by_inscription(inscription).await
    }

    /// Type specific alias for [`Repository::remove_validity`].
    async fn remove(&self, domain: &str) -> bool {
        self.remove_validity(domain).await
    }

    /// Type specific alias for [`Repository::remove_validity_by_inscription`].
    async fn remove_by_inscription(&self, inscription: &str) -> bool {
        self.remove_validity_by_inscription(inscription).await
    }

    /// Type specific alias for [`Repository::update_validity`].
    async fn update(&self, validity: ValidityTransfer) -> bool {
        self.update_validity(validity).await
    }

    /// Type specific alias for [`Repository::update_validity_by_inscription`].
    async fn update_by_inscription(&self, inscription: &str, validity: ValidityTransfer) -> bool {
        self.update_validity_by_inscription(inscription, validity)
            .await
    }
}
