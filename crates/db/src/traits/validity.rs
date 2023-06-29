use xdns_data::models::{Validity, ValidityTransfer};

pub trait ValidityRepository {
    fn get(&self, domain: &str) -> Vec<Validity>;
    fn add(&self, validity: &Validity) -> bool;
    fn remove(&self, validity: &Validity) -> bool;
    fn transfer(&self, validity: &ValidityTransfer) -> bool;
}