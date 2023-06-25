pub mod data;
pub mod domain;
pub mod subdomain;
pub mod validity;

pub mod prelude {
    pub use super::domain::Domain;
    pub use super::subdomain::{Class, SubDomain, Type};
}
