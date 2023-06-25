pub mod models;
pub mod parser;
pub mod traits;

pub mod prelude {
    pub use crate::models::prelude::*;
    pub use crate::traits::parser::Parser;
}
