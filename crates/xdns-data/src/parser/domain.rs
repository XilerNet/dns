use std::ops::RangeInclusive;

use shared::common::Result;
use shared::time::system_time_from_epoch_seconds;

use crate::models::domain::Domain;

const ASCII_LOWERCASE_START: u8 = 97;
const ASCII_LOWERCASE_END: u8 = 122;

const ASCII_NUMBERS_START: u8 = 48;
const ASCII_NUMBERS_END: u8 = 57;

const ASCII_HYPHEN: u8 = 45;

static ASCII_LOWERCASE_RANGE: RangeInclusive<u8> = ASCII_LOWERCASE_START..=ASCII_LOWERCASE_END;
static ASCII_NUMBERS_RANGE: RangeInclusive<u8> = ASCII_NUMBERS_START..=ASCII_NUMBERS_END;

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
    pub fn is_valid_edge_character(c: char) -> bool {
        c.is_ascii()
            && (ASCII_LOWERCASE_RANGE.contains(&(c as u8))
                || ASCII_NUMBERS_RANGE.contains(&(c as u8)))
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
        Domain::is_valid_edge_character(c) || (c.is_ascii() && c as u8 == ASCII_HYPHEN)
    }

    /// Checks if a top-level domain is present and gets it.
    ///
    /// # Arguments
    ///
    /// * `name` - The domain name to check.
    ///
    /// # Returns
    ///
    /// The top-level domain if present.
    pub fn get_tld(name: &str) -> Option<&str> {
        for (i, c) in name.chars().rev().enumerate() {
            if c == '.' {
                return Some(&name[name.len() - i..]);
            }
        }

        None
    }

    /// Checks if a top-level domain is valid.
    ///
    /// # Restrictions
    ///
    /// * The tld must be of the `o` domain space.
    ///
    /// # Arguments
    ///
    /// * `name` - The top-level domain to check.
    ///
    /// # Returns
    ///
    /// Whether the top-level domain is valid.
    pub fn is_tld_valid(name: &str) -> bool {
        Domain::get_tld(name).map_or(false, |tld| tld == "o")
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
        if !Domain::is_tld_valid(name) {
            return false;
        }

        // Remove the .o TLD.
        let name = &name[..name.len() - 2];

        if name.len() < 1 || name.len() > 254 {
            return false;
        }

        // If we only need to check one character, we can skip the check of the rest.
        if name.len() == 1 {
            return Domain::is_valid_edge_character(name.chars().next().unwrap());
        }

        let first_char = name.chars().next().unwrap();
        let last_char = name.chars().last().unwrap();
        let name_without_edge_chars = &name[1..name.len() - 1];

        Domain::is_valid_edge_character(first_char)
            && Domain::is_valid_edge_character(last_char)
            && name_without_edge_chars
                .chars()
                .all(Domain::is_valid_character)
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
        let mut parts = input.split_whitespace();

        if parts.next() != Some("DOMAIN") {
            return Err(format!("Input is not a domain record: {}", input).into());
        }

        let name = parts
            .next()
            .ok_or_else(|| format!("Domain record is missing name: {}", input))?;
        let valid_from = parts
            .next()
            .ok_or_else(|| format!("Domain record is missing valid_from: {}", input))?;

        if parts.next().is_some() {
            return Err(format!("Input is not a domain record: {}", input).into());
        }

        if !Domain::is_valid_domain_name(name) {
            return Err(format!("Invalid domain name: {}", name).into());
        }

        let valid_from = valid_from
            .parse::<u64>()
            .map_err(|e| format!("Invalid valid_from: {}: {}", valid_from, e))?;

        Ok(Self {
            name: name.to_owned(),
            valid_from: system_time_from_epoch_seconds(valid_from),
        })
    }
}
