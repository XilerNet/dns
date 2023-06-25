use shared::common::Error;
use crate::models::validity_transfer::ValidityTransfer;
use crate::prelude::Parser;

impl Parser for ValidityTransfer {
    fn parse(input: &str) -> Result<Self, Error> where Self: Sized {
        todo!()
    }
}