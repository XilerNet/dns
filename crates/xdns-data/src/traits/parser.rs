use shared::common::Error;

/// Represents a parser.
/// This can attempt to parse a string into a type.
pub trait Parser {
    fn parse(input: &str) -> Result<Self, Error>
    where
        Self: Sized;
}
