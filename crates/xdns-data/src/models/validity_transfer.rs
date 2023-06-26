use crate::models::credentials::Credentials;

#[derive(Debug)]
pub struct ValidityTransfer {
    pub domain: String,
    pub new_credentials: Option<Credentials>,
}
