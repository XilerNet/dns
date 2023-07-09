use shared::common::Result;

use crate::models::{Data, Domain, DomainDrop, Signature, SubDomain, Validity, ValidityTransfer};
use crate::traits::Parser;

#[derive(Debug)]
pub enum DomainAction {
    Domain(Domain),
    Subdomain(SubDomain),
    Drop(DomainDrop),
    Validity(Validity),
    ValidityTransfer(ValidityTransfer),
    Data(Data),
}

pub struct ActionParser {
    pub actions: Vec<DomainAction>,
    pub signature: Option<Signature>,
}

impl ActionParser {
    fn parse_action(content: &str) -> Result<DomainAction> {
        if let Ok(domain) = Domain::parse(content) {
            return Ok(DomainAction::Domain(domain));
        }

        if let Ok(subdomain) = SubDomain::parse(content) {
            return Ok(DomainAction::Subdomain(subdomain));
        }

        if let Ok(drop) = DomainDrop::parse(content) {
            return Ok(DomainAction::Drop(drop));
        }

        if let Ok(validity) = Validity::parse(content) {
            return Ok(DomainAction::Validity(validity));
        }

        if let Ok(validity_transfer) = ValidityTransfer::parse(content) {
            return Ok(DomainAction::ValidityTransfer(validity_transfer));
        }

        if let Ok(data) = Data::parse(content) {
            return Ok(DomainAction::Data(data));
        }

        Err("Invalid action".into())
    }

    fn parse_actions(contents: &Vec<String>) -> Result<Vec<DomainAction>> {
        let mut actions = Vec::with_capacity(contents.len());

        for content in contents {
            let action = Self::parse_action(content)?;
            actions.push(action);
        }

        Ok(actions)
    }
}

impl Parser for ActionParser {
    /// Attempts to parse content into its associated actions.
    /// Fails if the content is not valid in a valid structure, does not take in mind the signature.
    ///
    /// # Arguments
    ///
    /// * `input` - The content to parse.
    ///
    /// # Returns
    ///
    /// A `ActionParser` if the content was successfully parsed.
    fn parse(input: &str) -> Result<Self> {
        if input.is_empty() {
            return Err("Input is empty".into());
        }

        let signature = Signature::parse(input)?;
        let actions = Self::parse_actions(&signature.content)?;

        Ok(Self {
            actions,
            signature: signature.with_null_evaluation(),
        })
    }
}
