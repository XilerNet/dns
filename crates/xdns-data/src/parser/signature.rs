use shared::common::Result;

use crate::models::signature::Signature;
use crate::prelude::Parser;

impl Parser for Signature {
    /// Parses a signature and its content from a string.
    ///
    /// # Restrictions
    ///
    /// * The signature must be a valid hexadecimal string.
    /// * The signature must be present at the end of the input.
    /// * The signature must be separated from the content by a whitespace or a newline.
    /// * The content must not be empty.
    /// * The content is delimited by a newline.
    ///
    /// # Arguments
    ///
    /// * `input` - The string to parse.
    ///
    /// # Returns
    ///
    /// The parsed signature.
    fn parse(input: &str) -> Result<Self> {
        let parts = input
            .split_terminator('\n')
            .map(|part| part.trim())
            .filter(|part| !part.is_empty());

        let last_part = parts
            .clone()
            .last()
            .ok_or_else(|| format!("Invalid signature: {}", input))?
            .split_whitespace();

        let signature = last_part
            .clone()
            .last()
            .ok_or_else(|| format!("Signature not present: {}", input))?;

        if !signature.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(format!("Signature must consist of all hexadecimal characters: {}", input).into());
        }

        let last_content = last_part
            .take_while(|part| part != &signature)
            .map(|part| part.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        let mut content = parts
            .take_while(|part| part != &signature)
            .map(|part| part.to_string())
            .collect::<Vec<_>>();

        if !last_content.is_empty() {
            let length = content.len();
            content[length - 1] = last_content.to_string();
        }

        Ok(Self {
            content,
            signature: signature.to_string(),
        })
    }
}
