use crate::models::data::Data;
use crate::prelude::{Domain, Parser};
use shared::common::Result;

impl Parser for Data {
    /// Parse extra domain data, this is useful for when building custom protocols on top of DNS
    /// or more focused on the xdns to be able to extend the protocol for unseen changes.
    ///
    /// > NOTE: The data can contain spaces.
    ///
    /// # Restrictions
    ///
    /// * The domain must be valid. (See [`Domain::is_valid_domain_name`](Domain::is_valid_domain_name).)
    /// * The data must be in the format *(case sensitive)*: `DOMAIN-DATA <domain> <data>`.
    ///
    /// # Arguments
    ///
    /// * `input` - The input string to parse.
    ///
    /// # Returns
    ///
    /// The parsed data.
    fn parse(input: &str) -> Result<Self> {
        let mut parts = input.split_whitespace();

        if parts.next() != Some("DOMAIN-DATA") {
            return Err(format!("Input is not a domain data record: {}", input).into());
        }

        let domain = parts
            .next()
            .ok_or_else(|| format!("Domain data record is missing domain: {}", input))?;

        if !Domain::is_valid_domain_name(domain) {
            return Err(format!("Invalid domain: {}", domain).into());
        }

        let data = parts.collect::<Vec<&str>>().join(" ").as_bytes().to_vec();

        if data.is_empty() {
            return Err(format!("Domain data record is missing data: {}", input).into());
        }

        Ok(Self {
            domain: domain.to_string(),
            data,
        })
    }
}
