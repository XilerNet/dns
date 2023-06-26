use crate::models::algorithm::Algorithm;
use crate::traits::verify::Verify;
use shared::common::Result;

#[derive(Debug)]
pub struct Credentials {
    pub algorithm: Algorithm,
    pub public_key: String,
    pub verifier: Option<Box<dyn Verify>>,
}

impl Credentials {
    pub fn new(algorithm: Algorithm, public_key: String) -> Self {
        Self {
            algorithm,
            public_key,
            verifier: None,
        }
    }

    fn set_verifier(&mut self) -> Result<()> {
        let public_key_decoded = hex::decode(&self.public_key).unwrap();
        self.verifier = Some(self.algorithm.get_verifier(&public_key_decoded)?);
        Ok(())
    }

    /// Tries to verify a signature for a given message using the credentials.
    ///
    /// # Arguments
    ///
    /// * `data` - The message to verify.
    /// * `signature` - The signature to verify.
    ///
    /// # Returns
    ///
    /// Whether the signature is valid.
    pub fn try_is_valid(&mut self, data: &[u8], signature: &[u8]) -> Result<bool> {
        if self.verifier.is_none() {
            self.set_verifier()?;
        }
        let verifier = self.verifier.as_ref().unwrap();
        Ok(verifier.try_is_valid(data, signature)?)
    }
}
