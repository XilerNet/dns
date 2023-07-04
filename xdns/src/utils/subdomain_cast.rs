use dns_utils::prelude::DnsRecord;
use shared::common::Error;
use xdns_data::models::SubDomain;
use xdns_data::prelude::Type;

pub struct SubDomainCast(SubDomain);

impl SubDomainCast {
    fn get_domain(&self) -> String {
        if self.0.subdomain != "@" {
            return format!("{}.{}", self.0.subdomain, self.0.domain);
        }

        self.0.domain.to_string()
    }
}

impl From<SubDomain> for SubDomainCast {
    fn from(subdomain: SubDomain) -> Self {
        Self(subdomain)
    }
}

impl TryInto<DnsRecord> for SubDomainCast {
    type Error = Error;

    fn try_into(self) -> Result<DnsRecord, Self::Error> {
        Ok(match self.0.rtype {
            Type::A => DnsRecord::A {
                domain: self.get_domain(),
                ttl: self.0.ttl,
                addr: self.0.rdata.parse()?,
            },
            Type::AAAA => DnsRecord::AAAA {
                domain: self.get_domain(),
                ttl: self.0.ttl,
                addr: self.0.rdata.parse()?,
            },
            Type::NS => DnsRecord::NS {
                domain: self.get_domain(),
                ttl: self.0.ttl,
                host: self.0.rdata,
            },
            Type::CNAME => DnsRecord::CNAME {
                domain: self.get_domain(),
                ttl: self.0.ttl,
                host: self.0.rdata,
            },
            Type::MX => DnsRecord::MX {
                domain: self.get_domain(),
                ttl: self.0.ttl,
                priority: 0,
                host: self.0.rdata,
            },
        })
    }
}
