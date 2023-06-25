#[derive(Debug)]
pub struct ValidityTransfer {
    pub domain: String,
    pub new_credentials: Option<String>,
}