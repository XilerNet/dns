use xdns_data::models::Data;
use crate::Repository;

pub trait DataRepository {
    fn get(&self, domain: &str) -> Vec<Data>;
    fn add(&self, data: &Data) -> bool;
    fn remove(&self, data: &Data) -> bool;
}

impl <T: Repository> DataRepository for T {
    fn get(&self, domain: &str) -> Vec<Data> {
        self.get_data(domain)
    }

    fn add(&self, data: &Data) -> bool {
        self.add_data(data)
    }

    fn remove(&self, data: &Data) -> bool {
        self.remove_data(data)
    }
}