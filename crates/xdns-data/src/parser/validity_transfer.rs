use crate::models::credentials::Credentials;
use shared::common::Error;

use crate::models::validity_transfer::ValidityTransfer;
use crate::prelude::{Domain, Parser};

impl Parser for ValidityTransfer {
    /// Parse a validity transfer record.
    /// This record invalidates the validity of a domain and optionally sets a new validity.
    ///
    /// # Restrictions
    ///
    /// * The data must be in the format *(case sensitive)*: `DOMAIN-VALIDATE-TRANSFER <domain> [optional:<algorithm> <new public key>]`.
    /// * The domain must be valid. (See [`Domain::is_valid_domain_name`](Domain::is_valid_domain_name).)
    /// * The algorithm must be valid. (See [`Algorithm::is_valid_algorithm`](Algorithm::is_valid_algorithm).)
    ///
    /// # Arguments
    ///
    /// * `input` - The input string to parse.
    ///
    /// # Returns
    ///
    /// The parsed validity transfer record.
    fn parse(input: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut parts = input.split_whitespace();

        if parts.next() != Some("DOMAIN-VALIDATE-TRANSFER") {
            return Err(format!("Input is not a validity transfer record: {}", input).into());
        }

        let domain = parts
            .next()
            .ok_or_else(|| format!("Validity transfer record is missing domain: {}", input))?;

        let new_credentials = if let Some(algorithm) = parts.next() {
            let public_key = parts.next().ok_or_else(|| {
                format!("Validity transfer record is missing public key: {}", input)
            })?;

            Some(Credentials::new(algorithm.try_into()?, public_key.into()))
        } else {
            None
        };

        if parts.next().is_some() {
            return Err(format!("Validity transfer record has too many parts: {}", input).into());
        }

        if !Domain::is_valid_domain_name(domain) {
            return Err(format!("Invalid domain: {}", domain).into());
        }

        Ok(Self {
            domain: domain.to_string(),
            new_credentials,
        })
    }
}
