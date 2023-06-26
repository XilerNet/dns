use crate::models::credentials::Credentials;

#[derive(Debug)]
pub struct Validity {
    pub domain: String,
    pub credentials: Credentials,
}
