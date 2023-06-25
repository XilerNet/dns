use crate::models::domain::Domain;
use shared::common::Result;

const ASCII_LOWERCASE_START: u8 = 97;
const ASCII_LOWERCASE_END: u8 = 122;

const ASCII_NUMBERS_START: u8 = 48;
const ASCII_NUMBERS_END: u8 = 57;

const ASCII_HYPHEN: u8 = 45;


static ASCII_LOWERCASE_RANGE: std::ops::RangeInclusive<u8> = ASCII_LOWERCASE_START..=ASCII_LOWERCASE_END;
static ASCII_NUMBERS_RANGE: std::ops::RangeInclusive<u8> = ASCII_NUMBERS_START..=ASCII_NUMBERS_END;

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
    pub fn is_valid_edge_character(c: u8) -> bool {
        ASCII_LOWERCASE_RANGE.contains(&c) || ASCII_NUMBERS_RANGE.contains(&c)
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
    pub fn is_valid_character(c: char) -> bool {
        c.is_ascii() && (Domain::is_valid_edge_character(c as u8) || c as u8 == ASCII_HYPHEN)
    }

    /// Checks if a domain name is valid.
    ///
    /// # Restrictions
    ///
    /// * The domain must be at least one character long.
    /// * The domain must not contain any invalid characters. (See [`is_valid_character`](#method.is_valid_character).)
    /// * The domain first and last characters must be valid. (See [`is_valid_edge_character`](#method.is_valid_edge_character).)
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

    /// Parses a domain record from a string.
    ///
    /// # Restrictions
    ///
    /// * The domain must be valid. (See [`is_valid_domain_name`](#method.is_valid_domain_name).)
    /// * The domain record must be in the format *(case sensitive)*: `DOMAIN <name> <valid_from>`
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