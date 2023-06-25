#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
pub enum Algorithm {
    // TODO: Implement Ed25519
    Ed25519,
    // For the future: Dilithium5,
}

#[derive(Debug)]
pub struct Validity {
    pub domain: String,
    pub algorithm: Algorithm,
    pub key: Vec<u8>,
}
