use xdns_data::prelude::Type;

#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
pub enum QueryType {
    SUB(Type),
    UNKNOWN(u16),
}

impl QueryType {
    pub fn to_num(&self) -> u16 {
        match *self {
            QueryType::UNKNOWN(x) => x,
            QueryType::SUB(Type::A) => 1,
            QueryType::SUB(Type::NS) => 2,
            QueryType::SUB(Type::CNAME) => 5,
            QueryType::SUB(Type::MX) => 15,
            QueryType::SUB(Type::AAAA) => 28,
        }
    }

    pub fn from_num(num: u16) -> QueryType {
        match num {
            1 => QueryType::SUB(Type::A),
            2 => QueryType::SUB(Type::NS),
            5 => QueryType::SUB(Type::CNAME),
            15 => QueryType::SUB(Type::MX),
            28 => QueryType::SUB(Type::AAAA),
            _ => QueryType::UNKNOWN(num),
        }
    }
}