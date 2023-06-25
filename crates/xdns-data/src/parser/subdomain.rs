use shared::common::Result;

use crate::models::subdomain::*;
use crate::prelude::Domain;

const ASCII_DOT: u8 = 46;
const ASCII_HYPHEN: u8 = 45;
const ASCII_ASTERISK: u8 = 42;

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
        Domain::is_valid_character(c)
            || (c.is_ascii() && (c as u8 == ASCII_DOT || c as u8 == ASCII_ASTERISK))
    }

    /// Checks if a subdomain is valid.
    ///
    /// # Restrictions
    ///
    /// * The subdomain must not be empty.
    /// * The subdomain must not be longer than 63 characters.
    /// * The subdomain must not start or end with a `.` *(dot)* or `-` *(hyphen)*.
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
        if input == "." {
            return true;
        }

        if input.is_empty()
            || input.len() > 63
            || input.starts_with(ASCII_DOT as char)
            || !input.ends_with(ASCII_DOT as char)
            || input.starts_with(ASCII_HYPHEN as char)
            || input.ends_with(ASCII_HYPHEN as char)
        {
            return false;
        }

        // Remove the suffix `.`
        let input = &input[..input.len() - 1];

        todo!("Not implemented")
    }

    pub fn parse(input: &str) -> Result<Self> {
        todo!("Not implemented")
    }
}
