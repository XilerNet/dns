#![feature(async_fn_in_trait)]

pub use sources::SqliteRepository as Repository;
pub use traits::Repository as XDNSRepository;

pub mod sources;
pub mod traits;
