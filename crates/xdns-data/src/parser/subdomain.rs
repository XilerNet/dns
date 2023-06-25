use shared::common::Result;

use crate::models::subdomain::*;
use crate::prelude::Domain;

impl SubDomain {
    /// Checks if a character is valid for a subdomain.
    ///
    /// # Restrictions
    ///
    /// * The character must be a valid domain character or a `.` *(dot)*. See `Domain::is_valid_character`.
    ///
    /// # Arguments
    ///
    /// * `c` - The character to check.
    ///
    /// # Returns
    ///
    /// Whether the character is valid.
    pub fn is_valid_character(c: char) -> bool {
        Domain::is_valid_character(c) || (c.is_ascii() && (c == '.' || c == '*'))
    }

    /// Checks if a subdomain is valid.
    ///
    /// # Restrictions
    ///
    /// * The subdomain must not be empty.
    /// * The subdomain must not be longer than 63 characters.
    /// * The subdomain must not start or end *(ignoring the suffix `.`)* with a invalid edge character. See `Domain::is_valid_edge_character`.
    /// * A `.` *(dot)* must not surround a `-` *(hyphen)* or be adjacent to another `.` *(dot)*.
    /// * The subdomain must not contain any uppercase characters.
    /// * The subdomain must not contain any invalid characters. See `SubDomain::is_valid_character`.
    ///
    /// # Arguments
    ///
    /// * `input` - The subdomain to check.
    ///
    /// # Returns
    ///
    /// Whether the subdomain is valid.
    pub fn is_valid_subdomain(input: &str) -> bool {
        if input == "." || input == "*." {
            return true;
        }

        if input.is_empty()
            || input.len() > 63
            || !input.ends_with('.')
            || (!input.starts_with("*.")
                && !Domain::is_valid_edge_character(input.chars().next().unwrap()))
            || !Domain::is_valid_edge_character(input.chars().nth(input.len() - 2).unwrap())
            || input.contains("..")
            || input.contains(".-")
            || input.contains("-.")
        {
            return false;
        }

        input.chars().all(SubDomain::is_valid_character)
    }

    /// Parses a subdomain record from a string.
    ///
    /// > NOTE: This record should not include the blockchain reference nor should the signature be included.
    ///
    /// # Restrictions
    ///
    /// * The subdomain must be valid. (See [`is_valid_subdomain`](#method.is_valid_subdomain).)
    /// * The domain must be valid. (See [`Domain::is_valid_domain_name`](../prelude/struct.Domain.html#method.is_valid_domain_name).)
    /// * The subdomain record must be in the format *(case sensitive)*: `DNS <domain> <subdomain> <type> <class> <ttl> <rdata>`
    ///
    /// # Arguments
    ///
    /// * `input` - The string to parse.
    ///
    /// # Returns
    ///
    /// The parsed subdomain.
    pub fn parse(input: &str) -> Result<Self> {
        let mut parts = input.split_whitespace();

        if parts.next() != Some("DNS") {
            return Err(format!("Input is not a dns record: {}", input).into());
        }

        let domain = parts
            .next()
            .ok_or_else(|| format!("DNS record is missing domain: {}", input))?;
        let subdomain = parts
            .next()
            .ok_or_else(|| format!("DNS record is missing subdomain: {}", input))?;
        let rtype = parts
            .next()
            .ok_or_else(|| format!("DNS record is missing type: {}", input))?;
        let class = parts
            .next()
            .ok_or_else(|| format!("DNS record is missing class: {}", input))?;
        let ttl = parts
            .next()
            .ok_or_else(|| format!("DNS record is missing ttl: {}", input))?;
        let rdata = parts
            .next()
            .ok_or_else(|| format!("DNS record is missing rdata: {}", input))?;

        if parts.next().is_some() {
            return Err(format!("Input is not a dns record: {}", input).into());
        }

        if !Domain::is_valid_domain_name(domain) {
            return Err(format!("Invalid domain: {}", domain).into());
        }

        if !SubDomain::is_valid_subdomain(subdomain) {
            return Err(format!("Invalid subdomain: {}", subdomain).into());
        }

        Ok(Self {
            domain: domain.to_owned(),
            subdomain: subdomain.to_owned(),
            rtype: rtype.try_into()?,
            class: class.try_into()?,
            ttl: ttl.parse()?,
            rdata: rdata.to_owned(),
        })
    }
}
