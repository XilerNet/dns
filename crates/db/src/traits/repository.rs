use xdns_data::models::{Data, Domain, SubDomain, Validity, ValidityTransfer};

pub trait Repository {
    fn get_domain(&self, domain: &str) -> Vec<Domain>;
    fn add_domain(&self, domain: &Domain) -> bool;
    fn remove_domain(&self, domain: &Domain) -> bool;

    fn get_subdomain(&self, domain: &str) -> Vec<SubDomain>;
    fn add_subdomain(&self, subdomain: &SubDomain) -> bool;
    fn remove_subdomain(&self, subdomain: &SubDomain) -> bool;

    fn get_validity(&self, domain: &str) -> Vec<Validity>;
    fn add_validity(&self, validity: &Validity) -> bool;
    fn remove_validity(&self, validity: &Validity) -> bool;
    fn transfer_validity(&self, validity: &ValidityTransfer) -> bool;

    fn get_data(&self, domain: &str) -> Vec<Data>;
    fn add_data(&self, data: &Data) -> bool;
    fn remove_data(&self, data: &Data) -> bool;
}