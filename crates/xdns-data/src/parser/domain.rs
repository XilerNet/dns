use crate::models::domain::Domain;
use shared::common::Result;

impl Domain {
    /// Checks if a character is valid for the first or last character of a domain.
    ///
    /// # Restrictions
    ///
    /// * The character must not be uppercase.
    /// * The character must be a letter or number.
    ///
    /// # Arguments
    ///
    /// * `c` - The character to check.
    ///
    /// # Returns
    ///
    /// Whether the character is valid.
    fn is_valid_edge_character(c: char) -> bool {
        false
    }

    /// Checks if a character is valid for a domain.
    ///
    /// # Restrictions
    ///
    /// * The character must not be uppercase.
    /// * The character must be a letter, number, or hyphen.
    ///
    /// # Arguments
    ///
    /// * `c` - The character to check.
    ///
    /// # Returns
    ///
    /// Whether the character is valid.
    fn is_valid_character(c: char) -> bool {
        false
    }

    /// Checks if a domain name is valid.
    ///
    /// # Restrictions
    ///
    /// * The domain must be at least one character long.
    /// * The domain must not contain any invalid characters. (See `is_valid_character`.)
    /// * The domain first and last characters must be valid. (See `is_valid_edge_character`.)
    /// * The domain must not be longer than 254 characters.
    ///
    /// # Arguments
    ///
    /// * `name` - The domain name to check.
    ///
    /// # Returns
    ///
    /// Whether the domain name is valid.
    pub fn is_valid_domain_name(name: &str) -> bool {
        false
    }

    /// Parses a domain from a string.
    ///
    /// # Restrictions
    ///
    /// * The domain must be valid. (See `is_valid_domain_name`.)
    ///
    /// # Arguments
    ///
    /// * `input` - The string to parse.
    ///
    /// # Returns
    ///
    /// The parsed domain.
    pub fn parse(input: &str) -> Result<Self> {
        Err("Not implemented".into())
    }
}