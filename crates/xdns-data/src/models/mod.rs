pub mod domain;
pub mod subdomain;
pub mod validity;
pub mod data;

pub mod prelude {
    pub use super::domain::Domain;
    pub use super::subdomain::{SubDomain, Type, Class};

}
