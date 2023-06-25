use std::time::SystemTime;

#[derive(Debug)]
pub struct Domain {
    pub name: String,
    pub valid_from: SystemTime,
}