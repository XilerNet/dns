use crate::models::drop::DomainDrop;
use crate::prelude::Parser;
use shared::common::Result;

impl Parser for DomainDrop {
    /// Parse a drop record. This record can drop a subdomain or a domain all together.
    ///
    /// # Restrictions
    ///
    /// * The data must be in the format *(case sensitive)*: `DROP <inscription id>`.
    ///
    /// # Arguments
    ///
    /// * `input` - The input string to parse.
    ///
    /// # Returns
    ///
    /// The parsed drop record.
    fn parse(input: &str) -> Result<Self> {
        let mut parts = input.split_whitespace();

        if parts.next() != Some("DROP") {
            return Err(format!("Input is not a drop record: {}", input).into());
        }

        let inscription = parts
            .next()
            .ok_or_else(|| format!("Drop record is missing inscription: {}", input))?;

        if parts.next().is_some() {
            return Err(format!("Drop record has too many parts: {}", input).into());
        }

        Ok(Self {
            inscription: inscription.to_string(),
        })
    }
}
