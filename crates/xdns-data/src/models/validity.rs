use crate::models::algorithm::Algorithm;

#[derive(Debug)]
pub struct Credentials {
    pub algorithm: Algorithm,
    pub public_key: Vec<u8>,
}

#[derive(Debug)]
pub struct Validity {
    pub domain: String,
    pub credentials: Credentials,
}
