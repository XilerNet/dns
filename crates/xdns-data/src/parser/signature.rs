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
        let mut lines = input
            .split_terminator('\n')
            .map(str::trim)
            .filter(|&part| !part.is_empty());

        let last_line = lines.next_back().ok_or_else(|| format!("Invalid signature: {}", input))?;

        let last_line_parts: Vec<_> = last_line.split_whitespace().collect();
        let signature = last_line_parts.last().ok_or_else(|| format!("Invalid signature: {}", input))?;

        if !signature.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(format!("Signature must consist of all hexadecimal characters: {}", input).into());
        }

        let mut content: Vec<String> = lines.map(str::to_string).collect();
        let latest_line = last_line_parts[..last_line_parts.len() - 1].join(" ");

        if !latest_line.is_empty() {
            content.push(latest_line);
        }

        Ok(Self {
            content,
            signature: signature.to_string(),
        })
    }
}
