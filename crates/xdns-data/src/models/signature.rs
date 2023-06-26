use std::fmt::{Debug, Formatter};

pub struct Signature {
    pub signature: String,
}

impl Into<String> for Signature {
    /// Returns the signature.
    fn into(self) -> String {
        self.signature
    }
}

impl ToString for Signature {
    /// Returns the signature.
    fn to_string(&self) -> String {
        self.signature.clone()
    }
}

impl Debug for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.signature)
    }
}

impl AsRef<str> for Signature {
    fn as_ref(&self) -> &str {
        self.signature.as_ref()
    }
}
