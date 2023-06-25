use shared::common::Error;

#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
pub enum Algorithm {
    // TODO: Implement Ed25519
    Ed25519,
    // For the future: Dilithium5,
}

#[derive(Debug)]
pub struct Credentials {
    pub algorithm: Algorithm,
    pub public_key: Vec<u8>,
}

#[derive(Debug)]
pub struct Validity {
    pub domain: String,
    pub credentials: Credentials,
}

impl TryFrom<&str> for Algorithm {
    type Error = Error;

    /// Tries to convert a string to an [`Algorithm`](Algorithm).
    ///
    /// # Arguments
    ///
    /// * `value` - The string to convert.
    ///
    /// # Returns
    ///
    /// The algorithm.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ed25519" => Ok(Algorithm::Ed25519),
            _ => Err(format!("Unsupported algorithm: {}", value).into()),
        }
    }
}
