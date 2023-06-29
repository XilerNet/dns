use xdns_data::models::Data;

pub trait DataRepository {
    fn get(&self, domain: &str) -> Vec<Data>;
    fn add(&self, data: &Data) -> bool;
    fn remove(&self, data: &Data) -> bool;
}