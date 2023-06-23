use std::net::UdpSocket;
use dns_utils::prelude::*;
use shared::prelude::*;

#[test]
fn test_stub_resolver() -> Result<()> {
    // Perform an A query for google.com
    let qname = "google.com";
    let qtype = QueryType::A;

    // Using googles public DNS server
    let server = ("8.8.8.8", 53);

    // Bind a UDP socket to an arbitrary port
    let socket = UdpSocket::bind(("0.0.0.0", 43210))?;

    // Build our query packet. It's important that we remember to set the
    // `recursion_desired` flag. As noted earlier, the packet id is arbitrary.
    let mut packet = DnsPacket::new();

    packet.header.id = 6666;
    packet.header.questions = 1;
    packet.header.recursion_desired = true;
    packet
        .questions
        .push(DnsQuestion::new(qname.to_string(), qtype));

    // Use our new write method to write the packet to a buffer...
    let mut req_buffer = BytePacketBuffer::new();
    packet.write(&mut req_buffer)?;

    // ...and send it off to the server using our socket:
    socket.send_to(&req_buffer.buf[0..req_buffer.pos()], server)?;

    // To prepare for receiving the response, we'll create a new `BytePacketBuffer`,
    // and ask the socket to write the response directly into our buffer.
    let mut res_buffer = BytePacketBuffer::new();
    socket.recv_from(&mut res_buffer.buf)?;

    // As per the previous section, `DnsPacket::from_buffer()` is then used to
    // actually parse the packet after which we can print the response.
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
    assert_eq!(res_packet.header.answers, 1);
    assert_eq!(res_packet.header.authoritative_entries, 0);
    assert_eq!(res_packet.header.resource_entries, 0);

    assert_eq!(res_packet.questions.len(), 1);
    assert_eq!(res_packet.questions[0].name, qname);
    assert_eq!(res_packet.questions[0].qtype, qtype);

    assert_eq!(res_packet.answers.len(), 1);
    match &res_packet.answers[0] {
        DnsRecord::UNKNOWN { .. } => panic!("Got unknown record type"),
        DnsRecord::A { domain, addr: _, ttl } => {
            assert_eq!(domain, qname);
            assert!(ttl > &0);
        }
    }

    assert_eq!(res_packet.authorities.len(), 0);
    assert_eq!(res_packet.resources.len(), 0);
    Ok(())
}