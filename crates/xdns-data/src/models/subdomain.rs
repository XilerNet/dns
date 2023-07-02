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

    /// Tries to convert a string to a [`Type`](Type).
    ///
    /// # Arguments
    ///
    /// * `value` - The string to convert.
    ///
    /// # Returns
    ///
    /// The type.
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

    /// Tries to convert a string to a [`Class`](Class).
    ///
    /// # Arguments
    ///
    /// * `value` - The string to convert.
    ///
    /// # Returns
    ///
    /// The class.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "IN" => Ok(Self::IN),
            _ => Err(format!("Unsupported class: {}", value).into()),
        }
    }
}

impl Into<String> for Type {
    /// Converts a [`Type`](Type) to a string.
    ///
    /// # Arguments
    ///
    /// * `value` - The type to convert.
    ///
    /// # Returns
    ///
    /// The string.
    fn into(self) -> String {
        self.to_string()
    }
}

impl Into<String> for Class {
    /// Converts a [`Class`](Class) to a string.
    ///
    /// # Arguments
    ///
    /// * `value` - The class to convert.
    ///
    /// # Returns
    ///
    /// The string.
    fn into(self) -> String {
        self.to_string()
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Self::A => "A".to_string(),
            Self::NS => "NS".to_string(),
            Self::CNAME => "CNAME".to_string(),
            Self::MX => "MX".to_string(),
            Self::AAAA => "AAAA".to_string(),
        }
    }
}

impl ToString for Class {
    fn to_string(&self) -> String {
        match self {
            Self::IN => "IN".to_string(),
            Self::ANY => "ANY".to_string(),
        }
    }
}
