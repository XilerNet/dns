use xdns_data::models::{Validity, ValidityTransfer};
use crate::Repository;

pub trait ValidityRepository {
    fn get(&self, domain: &str) -> Vec<Validity>;
    fn add(&self, validity: &Validity) -> bool;
    fn remove(&self, validity: &Validity) -> bool;
    fn transfer(&self, validity: &ValidityTransfer) -> bool;
}

impl<T: Repository> ValidityRepository for T {
    fn get(&self, domain: &str) -> Vec<Validity> {
        self.get_validity(domain)
    }

    fn add(&self, validity: &Validity) -> bool {
        self.add_validity(validity)
    }

    fn remove(&self, validity: &Validity) -> bool {
        self.remove_validity(validity)
    }

    fn transfer(&self, validity: &ValidityTransfer) -> bool {
        self.transfer_validity(validity)
    }
}