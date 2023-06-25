extern crate shared;

pub mod bitwise;
pub mod packets;

pub mod prelude {
    pub use crate::packets::{
        byte_packet_buffer::BytePacketBuffer, dns_packet::DnsPacket, dns_question::DnsQuestion,
        dns_record::DnsRecord, query_type::QueryType, result_code::ResultCode,
    };
}
