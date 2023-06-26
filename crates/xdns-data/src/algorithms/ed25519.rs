use ed25519_dalek::{Signature, Verifier, VerifyingKey};

use crate::traits::verify::Verify;
use shared::common::Result;

#[derive(Debug)]
pub struct Ed25519 {
    pubic_key: VerifyingKey,
}

impl Verify for Ed25519 {
    fn new(public_key: &[u8; 32]) -> Self {
        Self {
            pubic_key: VerifyingKey::from_bytes(public_key).unwrap(),
        }
    }

    fn try_is_valid(&self, data: &[u8], signature: &[u8]) -> Result<bool> {
        let signature = Signature::try_from(signature)?;
        Ok(self.pubic_key.verify(data, &signature).is_ok())
    }
}
