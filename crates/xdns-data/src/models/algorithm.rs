extern crate paste;

use macro_rules_attribute::apply;
use shared::common::Error;

use crate::algorithms::dilithium::*;
use crate::algorithms::ed25519::Ed25519;
use crate::traits::verify::Verify;

// Dynamically build the name, verifier fetcher and attempt to convert from string
// based on the enum variants.
macro_rules! generate_algorithm_implementations {
     (
        $v:vis enum $name:ident {
            $($v_name:ident,)*
        }
    ) => {
         $v enum $name {
            $($v_name),*
        }

         paste::item! {
            impl $name {
                 /// Returns the name of the algorithm.
                 ///
                 /// # Returns
                 ///
                 /// The name of the algorithm.
                pub fn name(&self) -> &'static str {
                    match self {
                        $(Self::$v_name => stringify!([<$v_name:lower>]),)*
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
                pub fn get_verifier(&self, public_key: &[u8]) -> Result<Box<dyn Verify>, Error> {
                    match self {
                        $(Self::$v_name => Ok(Box::new($v_name::new(public_key.try_into()?)?)),)*
                    }
                }
            }

            impl TryFrom<&str> for $name {
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
                        $(stringify!([<$v_name:lower>]) => Ok(Self::$v_name),)*
                        _ => Err(format!("Unsupported algorithm: {}", value).into()),
                    }
                }
            }
        }
    };
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
#[apply(generate_algorithm_implementations!)]
pub enum Algorithm {
    Ed25519,
    Dilithium2,
    Dilithium2Aes,
    Dilithium3,
    Dilithium3Aes,
    Dilithium5,
    Dilithium5Aes,
}

impl TryFrom<String> for Algorithm {
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
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Algorithm::try_from(value.as_str())
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
