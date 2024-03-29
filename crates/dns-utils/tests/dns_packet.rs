use std::fs::File;
use std::io::Read;
use std::net::Ipv4Addr;
use std::str::FromStr;

use dns_utils::prelude::*;
use shared::prelude::*;
use xdns_data::prelude::Type::A;

#[test]
fn test_create_dns_packet_from_buffer() -> Result<()> {
    let mut f = File::open("data/response_packet.bin")?;
    let mut buffer = BytePacketBuffer::new();
    f.read(&mut buffer.buf)?;

    let packet = DnsPacket::from_buffer(&mut buffer)?;

    assert_eq!(packet.header.id, 53_033);
    assert!(packet.header.recursion_available);
    assert!(!packet.header.truncated_message);
    assert!(!packet.header.authoritative_answer);
    assert_eq!(packet.header.opcode, 0);
    assert!(packet.header.response);
    assert_eq!(packet.header.rescode, ResultCode::NOERROR);
    assert!(!packet.header.checking_disabled);
    assert!(packet.header.authed_data);
    assert!(!packet.header.z);
    assert!(packet.header.recursion_available);
    assert_eq!(packet.header.questions, 1);
    assert_eq!(packet.header.answers, 2);
    assert_eq!(packet.header.authoritative_entries, 0);
    assert_eq!(packet.header.resource_entries, 0);

    assert_eq!(packet.questions.len(), 1);
    assert_eq!(packet.answers.len(), 2);
    assert_eq!(packet.authorities.len(), 0);
    assert_eq!(packet.resources.len(), 0);

    assert_eq!(packet.questions[0].name, "xiler.net");
    assert_eq!(packet.questions[0].qtype, QueryType::SUB(A));

    if let DnsRecord::A { domain, addr, ttl } = &packet.answers[0] {
        assert_eq!(domain, "xiler.net");
        assert_eq!(*addr, Ipv4Addr::from_str("104.21.4.85")?);
        assert_eq!(*ttl, 300);
    }

    if let DnsRecord::A { domain, addr, ttl } = &packet.answers[1] {
        assert_eq!(domain, "xiler.net");
        assert_eq!(*addr, Ipv4Addr::from_str("172.67.131.221")?);
        assert_eq!(*ttl, 300);
    }

    Ok(())
}
