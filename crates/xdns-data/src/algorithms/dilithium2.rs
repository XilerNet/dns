use shared::common::Result;
use std::fmt::{self, Debug};

use crate::traits::verify::Verify;
use pqcrypto::prelude::*;
use pqcrypto_dilithium::dilithium2::{DetachedSignature, PublicKey};
use pqcrypto_dilithium::dilithium2_verify_detached_signature;

pub struct Dilithium2 {
    pubic_key: PublicKey,
}

impl Debug for Dilithium2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dilithium2()")
    }
}

impl Verify for Dilithium2 {
    fn new(public_key: &[u8]) -> Result<Self> {
        Ok(Self {
            pubic_key: PublicKey::from_bytes(public_key)?,
        })
    }

    fn try_is_valid(&self, data: &[u8], signature: &[u8]) -> Result<bool> {
        let signature = DetachedSignature::from_bytes(signature)?;
        Ok(dilithium2_verify_detached_signature(&signature, data, &self.pubic_key).is_ok())
    }
}
