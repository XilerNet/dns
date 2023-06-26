use crate::models::algorithm::Algorithm;
use crate::traits::verify::Verify;

#[derive(Debug)]
pub struct Credentials {
    pub algorithm: Algorithm,
    pub public_key: String,
    pub verifier: Option<Box<dyn Verify>>,
}

impl Credentials {
    pub fn new(algorithm: Algorithm, public_key: String) -> Self {
        Self {
            algorithm,
            public_key,
            verifier: None,
        }
    }

    fn set_verifier(&mut self) {
        todo!("set_verifier")
    }
}
