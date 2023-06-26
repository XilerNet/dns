use crate::algorithms::ed25519::Ed25519;
use crate::traits::verify::Verify;
use shared::common::Error;

#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
pub enum Algorithm {
    // TODO: Implement Ed25519
    Ed25519,
    // For the future: Dilithium5,
}

impl Algorithm {
    /// Returns the name of the algorithm.
    ///
    /// # Returns
    ///
    /// The name of the algorithm.
    pub fn name(&self) -> &'static str {
        match self {
            Algorithm::Ed25519 => "ed25519",
        }
    }

    /// Returns a [`Verify`](Verify) instance for the algorithm.
    /// This instance can be used to verify signatures.
    ///
    /// # Arguments
    ///
    /// * `public_key` - The public key to use for verification.
    ///
    /// # Returns
    ///
    /// The [`Verify`](Verify) instance.
    pub fn get_verifier(&self, public_key: &[u8; 32]) -> Box<dyn Verify> {
        match self {
            Algorithm::Ed25519 => Box::new(Ed25519::new(public_key)),
        }
    }
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

impl Into<&str> for Algorithm {
    /// Returns the name of the algorithm.
    fn into(self) -> &'static str {
        self.name()
    }
}

impl ToString for Algorithm {
    /// Returns the name of the algorithm.
    fn to_string(&self) -> String {
        self.name().to_string()
    }
}

impl Into<String> for Algorithm {
    /// Returns the name of the algorithm.
    fn into(self) -> String {
        self.to_string()
    }
}
