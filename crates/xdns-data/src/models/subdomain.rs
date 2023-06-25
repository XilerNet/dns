use shared::common::Error;

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
    pub rdata: String,
}

impl TryFrom<&str> for Type {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Self::A),
            "NS" => Ok(Self::NS),
            "CNAME" => Ok(Self::CNAME),
            "MX" => Ok(Self::MX),
            "AAAA" => Ok(Self::AAAA),
            _ => Err(format!("Unsupported type: {}", value).into()),
        }
    }
}

impl TryFrom<&str> for Class {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "IN" => Ok(Self::IN),
            _ => Err(format!("Unsupported class: {}", value).into()),
        }
    }
}
