use shared::common::Result;

use crate::models::signature::Signature;
use crate::prelude::Parser;

impl Parser for Signature {
    /// Parses a signature from a string.
    ///
    /// # Restrictions
    ///
    /// * The signature must be a valid hexadecimal string.
    /// * The signature must be present at the end of the input.
    ///
    /// # Arguments
    ///
    /// * `input` - The string to parse.
    ///
    /// # Returns
    ///
    /// The parsed signature.
    fn parse(input: &str) -> Result<Self> {
        let signature = input
            .split_whitespace()
            .last()
            .ok_or_else(|| format!("Signature is missing: {}", input))?;

        if !signature.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(format!("Signature is not a valid hexadecimal string: {}", signature).into());
        }

        let content = input[..input.len() - signature.len()].trim();

        if content.is_empty() {
            return Err(format!("Signature is missing content: {}", input).into());
        }

        Ok(Self {
            signature: signature.to_string(),
            content: content.to_string(),
        })
    }
}
