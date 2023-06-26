use shared::common::Result;

use crate::models::credentials::Credentials;
use crate::models::validity::Validity;
use crate::prelude::Domain;
use crate::traits::parser::Parser;

impl Parser for Validity {
    /// Parses a validity record from a string.
    ///
    /// # Restrictions
    ///
    /// * The algorithm mustn't contain spaces and be valid. (See [`Algorithm::try_from`](../validity/enum.Algorithm.html#method.try_from).)
    /// * The domain must be valid. (See [`Domain::is_valid_domain_name`](../prelude/struct.Domain.html#method.is_valid_domain_name).)
    /// * The validity record must be in the format *(case sensitive)*: `DOMAIN-VALIDITY <domain> <algorithm> <key>`
    ///
    /// # Arguments
    ///
    /// * `input` - The string to parse.
    ///
    /// # Returns
    ///
    /// The parsed validity record.
    fn parse(input: &str) -> Result<Self> {
        let mut parts = input.split_whitespace();

        if parts.next() != Some("DOMAIN-VALIDITY") {
            return Err(format!("Input is not a validity record: {}", input).into());
        }

        let domain = parts
            .next()
            .ok_or_else(|| format!("Validity record is missing domain: {}", input))?;
        let algorithm = parts
            .next()
            .ok_or_else(|| format!("Validity record is missing algorithm: {}", input))?;
        let key = parts
            .next()
            .ok_or_else(|| format!("Validity record is missing key: {}", input))?;

        if parts.next().is_some() {
            return Err(format!("Input is not a validity record: {}", input).into());
        }

        if !Domain::is_valid_domain_name(domain) {
            return Err(format!("Invalid domain: {}", domain).into());
        }

        Ok(Self {
            domain: domain.to_owned(),
            credentials: Credentials::new(algorithm.try_into()?, key.into()),
        })
    }
}
