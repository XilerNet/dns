extern crate dns_utils;
extern crate shared;

use std::net::UdpSocket;

use dns_utils::prelude::*;
use shared::prelude::*;

const SERVER: (&str, u16) = ("1.1.1.1", 53);
const PORT: u16 = 53;

fn lookup(qname: &str, qtype: QueryType, packet: Option<DnsPacket>) -> Result<DnsPacket> {
    println!("Looking up {:?} {:?}", qname, qtype);

    let mut packet = match packet {
        Some(packet) => packet,
        None => {
            let mut packet = DnsPacket::new();

            packet.header.id = 6666;
            packet.header.questions = 1;
            packet.header.recursion_desired = true;
            packet
                .questions
                .push(DnsQuestion::new(qname.to_string(), qtype));

            packet
        }
    };

    if qname.ends_with(".o") {
        // TODO: Get the packet from the db
        if qname == "xiler.o" {
            let mut packet = DnsPacket {
                header: packet.header,
                questions: packet.questions,
                original_questions: None,
                answers: vec![DnsRecord::CNAME {
                    domain: qname.to_string(),
                    host: "xiler.net".to_string(),
                    ttl: 64,
                }],
                authorities: vec![],
                resources: vec![],
            };

            if let Some(record) = packet.answers.last() {
                return if record.type_of() == qtype {
                    Ok(packet.make_returnable())
                } else if let Some(host) = record.get_host() {
                    // TODO: Prevent clone usage here
                    packet.original_questions = Some(packet.questions);
                    packet.questions = vec![DnsQuestion::new(host.to_string(), qtype)];
                    lookup(host, qtype, Some(packet.clone()))
                } else {
                    Ok(packet.make_returnable())
                };
            }

            return Ok(packet.make_returnable());
        }

        Ok(packet)
    } else {
        let mut req_buffer = BytePacketBuffer::new();
        packet.write(&mut req_buffer)?;

        let socket = UdpSocket::bind(("0.0.0.0", 43210))?;
        socket.send_to(&req_buffer.buf[0..req_buffer.pos()], SERVER)?;

        let mut res_buffer = BytePacketBuffer::new();
        socket.recv_from(&mut res_buffer.buf)?;

        let res_packet = DnsPacket::from_buffer(&mut res_buffer);

        match res_packet {
            Ok(res_packet) => {
                for answer in res_packet.answers {
                    packet.answers.push(answer);
                }
                Ok(packet.make_returnable())
            }
            e @ Err(_) => e,
        }
    }
}

/// Handle a single incoming packet
fn handle_query(socket: &UdpSocket) -> Result<()> {
    // With a socket ready, we can go ahead and read a packet. This will
    // block until one is received.
    let mut req_buffer = BytePacketBuffer::new();

    // The `recv_from` function will write the data into the provided buffer,
    // and return the length of the data read as well as the source address.
    // We're not interested in the length, but we need to keep track of the
    // source in order to send our reply later on.
    let (_, src) = socket.recv_from(&mut req_buffer.buf)?;

    // Next, `DnsPacket::from_buffer` is used to parse the raw bytes into
    // a `DnsPacket`.
    let mut request = DnsPacket::from_buffer(&mut req_buffer)?;

    // Create and initialize the response packet
    let mut packet = DnsPacket::new();
    packet.header.id = request.header.id;
    packet.header.recursion_desired = true;
    packet.header.recursion_available = true;
    packet.header.response = true;

    // In the normal case, exactly one question is present
    if let Some(question) = request.questions.pop() {
        println!("Received query: {:?}", question);

        // Since all is set up and as expected, the query can be forwarded to the
        // target SERVER. There's always the possibility that the query will
        // fail, in which case the `SERVFAIL` response code is set to indicate
        // as much to the client. If rather everything goes as planned, the
        // question and response records as copied into our response packet.
        if let Ok(result) = lookup(&question.name, question.qtype, None) {
            packet.questions.push(question);
            packet.header.rescode = result.header.rescode;

            for rec in result.answers {
                println!("Answer: {:?}", rec);
                packet.answers.push(rec);
            }
            for rec in result.authorities {
                println!("Authority: {:?}", rec);
                packet.authorities.push(rec);
            }
            for rec in result.resources {
                println!("Resource: {:?}", rec);
                packet.resources.push(rec);
            }
        } else {
            packet.header.rescode = ResultCode::SERVFAIL;
        }
    }
    // Being mindful of how unreliable input data from arbitrary senders can be, we
    // need make sure that a question is actually present. If not, we return `FORMERR`
    // to indicate that the sender made something wrong.
    else {
        packet.header.rescode = ResultCode::FORMERR;
    }

    let mut res_buffer = BytePacketBuffer::new();
    packet.write(&mut res_buffer)?;

    let len = res_buffer.pos();
    let data = res_buffer.get_range(0, len)?;

    socket.send_to(data, src)?;

    Ok(())
}

fn main() -> Result<()> {
    let socket = UdpSocket::bind(("127.0.0.1", PORT))?;
    println!("DNS proxy listening on port {}", PORT);

    loop {
        match handle_query(&socket) {
            Ok(_) => {}
            Err(e) => eprintln!("An error occurred: {}", e),
        }
    }
}
