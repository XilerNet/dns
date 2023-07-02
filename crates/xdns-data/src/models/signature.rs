use crate::models::Credentials;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Signature {
    pub content: Vec<String>,
    pub signature: String,
    pub last_id: Option<String>,
    pub(crate) last_id_on_separate_line: bool,
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
        let separator = if self.last_id_on_separate_line {
            "\n"
        } else {
            " "
        };
        let signature_content = self.content.join("\n")
            + separator
            + self
                .last_id
                .as_ref()
                .unwrap_or(&"null".to_string())
                .as_str();

        let is_valid = credentials.try_is_valid(signature_content.as_bytes(), &signature);

        is_valid.is_ok() && is_valid.unwrap()
    }

    /// Checks whether the signature is considered null.
    ///
    /// # Returns
    ///
    /// Whether the signature is null.
    pub fn is_null(&self) -> bool {
        self.signature.is_empty() || self.signature == "null"
    }

    /// Check if the signature is null, and if so, return None.
    ///
    /// # Returns
    ///
    /// The signature if it is not null, None otherwise.
    pub fn with_null_evaluation(self) -> Option<Self> {
        if self.is_null() {
            None
        } else {
            Some(self)
        }
    }
}
