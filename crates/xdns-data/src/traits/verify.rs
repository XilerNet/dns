use shared::common::Result;
use std::fmt::Debug;

pub trait Verify: Debug {
    /// Creates a new instance of the verifier.
    /// This instance can be used to verify signatures.
    ///
    /// # Arguments
    ///
    /// * `public_key` - The public key to use for verification.
    ///
    /// # Returns
    ///
    /// The verifier instance.
    fn new(public_key: &[u8; 32]) -> Result<Self>
    where
        Self: Sized;

    /// Attempts to verify the signature.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to verify.
    /// * `signature` - The signature to verify.
    ///
    /// # Returns
    ///
    /// Whether the signature is valid.
    fn try_is_valid(&self, data: &[u8], signature: &[u8]) -> Result<bool>;
}
