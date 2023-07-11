extern crate paste;

use shared::common::Result;
use std::fmt::{self, Debug};

use crate::traits::verify::Verify;
use pqcrypto::prelude::*;

macro_rules! add_dilithium {
    ($version:ident) => {
        paste::item! {
            use pqcrypto_dilithium::[<$version:lower _verify_detached_signature>];

            pub struct [<$version:camel>] {
                pubic_key: pqcrypto_dilithium::[<$version:lower>]::PublicKey,
            }

            impl Debug for [<$version:camel>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "[<$version:camel>]()")
                }
            }

            impl Verify for [<$version:camel>] {
                fn new(public_key: &[u8]) -> Result<Self> {
                    Ok(Self {
                        pubic_key: pqcrypto_dilithium::[<$version:lower>]::PublicKey::from_bytes(public_key)?,
                    })
                }

                fn try_is_valid(&self, data: &[u8], signature: &[u8]) -> Result<bool> {
                    let signature = pqcrypto_dilithium::[<$version:lower>]::DetachedSignature::from_bytes(signature)?;
                    Ok([<$version:lower _verify_detached_signature>](&signature, data, &self.pubic_key).is_ok())
                }
            }
        }
    }
}

add_dilithium!(dilithium2);
add_dilithium!(dilithium2Aes);
add_dilithium!(dilithium3);
add_dilithium!(dilithium3Aes);
add_dilithium!(dilithium5);
add_dilithium!(dilithium5Aes);
