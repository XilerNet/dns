extern crate shared;

pub mod bitwise;
pub mod packets;

pub mod prelude {
    pub use crate::packets::{
        byte_packet_buffer::BytePacketBuffer,
        dns_packet::DnsPacket,
        result_code::ResultCode,
        query_type::QueryType
    };
}