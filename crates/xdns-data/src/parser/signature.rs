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
        let mut last_id_on_separate_line = false;
        let mut lines = input
            .split_terminator('\n')
            .map(str::trim)
            .filter(|&part| !part.is_empty());

        let last_line = lines.next_back().ok_or_else(|| format!("Signature does not have content: {}", input))?;

        let mut last_line_parts = last_line.split_whitespace();
        let signature = last_line_parts.next_back().ok_or_else(|| format!("Signature is missing signature: {}", input))?;
        let last_id = last_line_parts.next_back().ok_or_else(|| format!("Signature is missing the last id: {}", input))?;

        if !signature.chars().all(|c| c.is_ascii_hexdigit()) && signature != "null" {
            return Err(format!("Signature must consist of all hexadecimal characters: {}", input).into());
        }

        let mut content: Vec<String> = lines.map(str::to_string).collect();
        let latest_line = last_line_parts.collect::<Vec<_>>().join(" ");

        if !latest_line.is_empty() {
            content.push(latest_line);
        } else {
            last_id_on_separate_line = true;
        }

        let last_id = if last_id == "null" {
            None
        } else {
            Some(last_id.to_string())
        };

        Ok(Self {
            content,
            last_id,
            last_id_on_separate_line,
            signature: signature.to_string(),
        })
    }
}
