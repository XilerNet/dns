use xdns_data::models::{Data, Domain, SubDomain, Validity, ValidityTransfer};
use crate::traits::Repository;

pub struct SqliteRepository {

}

impl Repository for SqliteRepository {
    fn get_domain(&self, domain: &str) -> Vec<Domain> {
        todo!()
    }

    fn add_domain(&self, domain: &Domain) -> bool {
        todo!()
    }

    fn remove_domain(&self, domain: &Domain) -> bool {
        todo!()
    }

    fn get_subdomain(&self, domain: &str) -> Vec<SubDomain> {
        todo!()
    }

    fn add_subdomain(&self, subdomain: &SubDomain) -> bool {
        todo!()
    }

    fn remove_subdomain(&self, subdomain: &SubDomain) -> bool {
        todo!()
    }

    fn get_validity(&self, domain: &str) -> Vec<Validity> {
        todo!()
    }

    fn add_validity(&self, validity: &Validity) -> bool {
        todo!()
    }

    fn remove_validity(&self, validity: &Validity) -> bool {
        todo!()
    }

    fn transfer_validity(&self, validity: &ValidityTransfer) -> bool {
        todo!()
    }

    fn get_data(&self, domain: &str) -> Vec<Data> {
        todo!()
    }

    fn add_data(&self, data: &Data) -> bool {
        todo!()
    }

    fn remove_data(&self, data: &Data) -> bool {
        todo!()
    }
}