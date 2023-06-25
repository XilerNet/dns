#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
pub enum Type {
    A,
    NS,
    CNAME,
    MX,
    AAAA,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
pub enum Class {
    IN,
    ANY,
}

#[derive(Debug)]
pub struct SubDomain {
    pub domain: String,
    pub subdomain: String,
    pub rtype: Type,
    pub class: Class,
    pub ttl: u32,
    pub rdata: Vec<u8>,
    pub id: String,
}