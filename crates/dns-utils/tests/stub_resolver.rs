use dns_utils::prelude::*;
use shared::prelude::*;
use std::net::UdpSocket;
use xdns_data::prelude::Type::A;

#[test]
fn test_stub_resolver() -> Result<()> {
    let qname = "dns-test-proxy-root.xiler.net";
    let qtype = QueryType::SUB(A);

    let server = ("8.8.8.8", 53);
    let socket = UdpSocket::bind(("0.0.0.0", 43210))?;

    let mut packet = DnsPacket::new();
    packet.header.id = 6666;
    packet.header.questions = 1;
    packet.header.recursion_desired = true;
    packet
        .questions
        .push(DnsQuestion::new(qname.to_string(), qtype));

    let mut req_buffer = BytePacketBuffer::new();
    packet.write(&mut req_buffer)?;
    socket.send_to(&req_buffer.buf[0..req_buffer.pos()], server)?;

    let mut res_buffer = BytePacketBuffer::new();
    socket.recv_from(&mut res_buffer.buf)?;

    let res_packet = DnsPacket::from_buffer(&mut res_buffer)?;

    assert_eq!(res_packet.header.id, 6666);
    assert!(res_packet.header.recursion_desired);
    assert!(!res_packet.header.truncated_message);
    assert!(!res_packet.header.authoritative_answer);
    assert_eq!(res_packet.header.opcode, 0);
    assert!(res_packet.header.response);
    assert_eq!(res_packet.header.rescode, ResultCode::NOERROR);
    assert!(!res_packet.header.checking_disabled);
    assert!(!res_packet.header.authed_data);
    assert!(!res_packet.header.z);
    assert!(res_packet.header.recursion_available);
    assert_eq!(res_packet.header.questions, 1);
    assert!(res_packet.header.answers > 0);
    assert_eq!(res_packet.header.authoritative_entries, 0);
    assert_eq!(res_packet.header.resource_entries, 0);

    assert_eq!(res_packet.questions.len(), 1);
    assert_eq!(res_packet.questions[0].name, qname);
    assert_eq!(res_packet.questions[0].qtype, qtype);

    assert!(res_packet.answers.len() > 0);
    assert!(matches!(
        res_packet.answers.last(),
        Some(DnsRecord::A { .. })
    ));

    assert_eq!(res_packet.authorities.len(), 0);
    assert_eq!(res_packet.resources.len(), 0);
    Ok(())
}
