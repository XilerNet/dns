use std::fmt::Debug;
use crate::models::Credentials;

#[derive(Debug)]
pub struct Signature {
    pub content: Vec<String>,
    pub signature: String,
}

impl Signature {
    /// Checks whether the signature is valid for the given credentials.
    ///
    /// # Arguments
    ///
    /// * `credentials` - The credentials to check the signature against.
    ///
    /// # Returns
    ///
    /// Whether the signature is valid.
    pub fn is_valid(&self, mut credentials: Credentials) -> bool {
        let signature = hex::decode(&self.signature);

        if signature.is_err() {
            return false;
        }

        let signature = signature.unwrap();

        let is_valid = credentials.try_is_valid(
            self.content.join("\n").as_bytes(),
            &signature,
        );

        is_valid.is_ok() && is_valid.unwrap()
    }
}
