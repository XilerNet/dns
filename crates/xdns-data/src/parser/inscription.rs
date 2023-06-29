use shared::common::Result;

use crate::models::{Data, Domain, DomainDrop, Signature, SubDomain, Validity, ValidityTransfer};
use crate::traits::Parser;

pub enum Inscription {
    Domain(Domain),
    Subdomain(SubDomain),
    Drop(DomainDrop),
    Validity(Validity),
    ValidityTransfer(ValidityTransfer),
    Data(Data),
}

pub struct InscriptionParser {
    pub inscriptions: Vec<Inscription>,
    pub signature: Signature,
}

impl Parser for InscriptionParser {
    fn parse(input: &str) -> Result<Self> {
        todo!()
    }
}