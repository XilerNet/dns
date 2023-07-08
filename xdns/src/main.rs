mod utils;

extern crate db;
extern crate dns_utils;
extern crate shared;

use crate::utils::subdomain_cast::SubDomainCast;
use crate::utils::ExpiringMultiValueHashMap;
use async_recursion::async_recursion;
use db::{Repository, XDNSRepository};
use dns_utils::prelude::*;
use lazy_static::lazy_static;
use rayon::prelude::*;
use shared::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use xdns_data::prelude::Type;

const SERVER: (&str, u16) = ("1.1.1.1", 53);
const PORT: u16 = 53;
const BLACKLIST_FILE: &str = "blacklist.txt";

lazy_static! {
    static ref CACHE: Mutex<ExpiringMultiValueHashMap<String, DnsRecord>> =
        Mutex::new(ExpiringMultiValueHashMap::new());
    static ref BLACKLIST: HashSet<String> =
        read_blacklisted_domains().expect("Failed to read blacklist file, make sure it exists");
}

fn read_blacklisted_domains() -> Result<HashSet<String>> {
    let mut file = File::open(BLACKLIST_FILE)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let domains: HashSet<String> = contents
        .lines()
        .par_bridge()
        .into_par_iter()
        .map(|line| line.to_string())
        .collect();

    Ok(domains)
}

fn get_blacklist() -> &'static HashSet<String> {
    &BLACKLIST
}

#[async_recursion]
async fn lookup(qname: &str, qtype: QueryType, packet: Option<DnsPacket>) -> Result<DnsPacket> {
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

    if get_blacklist().contains(qname) {
        println!("Attempted lookup of blacklisted domain: {:?}", qname);
        packet.header.rescode = ResultCode::REFUSED;
        return Ok(packet.make_returnable());
    }

    let cache = CACHE.lock().await;
    if cache.contains_key(qname) {
        println!("Cache hit for {:?}", qname);
        let now = Instant::now();
        let cached_records: Vec<DnsRecord> = cache
            .get(qname)
            .unwrap()
            .iter()
            .map(|(record, ttl)| {
                let mut record = record.clone();
                let passed = ttl
                    .checked_duration_since(now)
                    .unwrap_or(Duration::from_secs(0));
                record.set_ttl(passed.as_secs() as u32);
                record
            })
            .collect();

        packet
            .answers
            .extend(cached_records.iter().map(|record| record.clone()));

        let resolved_domains = cached_records
            .iter()
            .map(|record| record.get_domain())
            .collect::<HashSet<&str>>();

        let resolved_cnames: HashSet<&str> = cached_records
            .iter()
            .filter(|record| record.type_of() == QueryType::SUB(Type::CNAME))
            .filter(|record| resolved_domains.contains(&record.get_host().unwrap()))
            .map(|record| record.get_domain())
            .collect();

        drop(cache);

        if qtype != QueryType::SUB(Type::CNAME) {
            for record in cached_records.iter() {
                if record.type_of() == QueryType::SUB(Type::CNAME) {
                    if resolved_cnames.contains(&record.get_domain()) {
                        continue;
                    }

                    let mut tmp_packet = packet.clone();
                    tmp_packet.questions = vec![DnsQuestion::new(
                        record.get_host().unwrap().to_string(),
                        qtype,
                    )];
                    tmp_packet.answers = Vec::new();
                    let res =
                        lookup(record.get_host().unwrap(), qtype, Some(tmp_packet.clone())).await;

                    if let Ok(res) = res {
                        packet.answers.extend(res.answers);
                    }
                }
            }
        }

        return Ok(packet.make_returnable());
    }
    drop(cache);

    if qname.ends_with(".o") {
        let db = Repository::new().await;
        let segments = qname.split(".").collect::<Vec<&str>>();
        let domain = segments[segments.len() - 2..].join(".");
        let mut subdomain = segments[..segments.len() - 2].join(".") + ".";

        if subdomain.trim() == "." {
            subdomain = "@.".to_string();
        }

        let subdomains = match db.get_subdomain(&domain, &subdomain).await {
            Ok(subdomains) => subdomains,
            Err(e) => {
                println!("{:?}", e);
                return Ok(packet);
            }
        };

        if subdomains.len() > 0 {
            let answers: Vec<DnsRecord> = subdomains
                .into_iter()
                .map(|s| SubDomainCast::from(s.1))
                .map(|s| s.try_into())
                .collect::<Result<Vec<DnsRecord>>>()?;

            packet.answers = answers;

            for answer in packet.answers.iter() {
                CACHE.lock().await.insert(
                    answer.get_domain().to_string(),
                    answer.clone(),
                    Duration::from_secs(answer.get_ttl() as u64),
                );
            }

            if qtype != QueryType::SUB(Type::CNAME) {
                let mut cname_resolves = Vec::new();

                for record in packet.answers.iter() {
                    if record.type_of() == QueryType::SUB(Type::CNAME) {
                        let mut packet = packet.clone();
                        packet.questions = vec![DnsQuestion::new(
                            record.get_host().unwrap().to_string(),
                            qtype,
                        )];
                        packet.answers = Vec::new();
                        let res =
                            lookup(record.get_host().unwrap(), qtype, Some(packet.clone())).await;

                        if let Ok(res) = res {
                            cname_resolves.push(res);
                        }
                    }
                }

                for cname_resolve in cname_resolves {
                    packet.answers.extend(cname_resolve.answers);
                }
            }
            let packet = packet.make_returnable();

            return Ok(packet);
        }

        Ok(packet)
    } else {
        let mut req_buffer = BytePacketBuffer::new();
        packet.write(&mut req_buffer)?;

        let socket = UdpSocket::bind(("0.0.0.0", 43210)).await?;
        socket
            .send_to(&req_buffer.buf[0..req_buffer.pos()], SERVER)
            .await?;

        let mut res_buffer = BytePacketBuffer::new();
        socket.recv_from(&mut res_buffer.buf).await?;

        let res_packet = DnsPacket::from_buffer(&mut res_buffer)?;
        packet.answers.extend(res_packet.answers.clone());

        let packet = packet.make_returnable();

        for answer in packet.answers.iter() {
            println!("Caching (not .o) {:?}", answer.get_domain());
            CACHE.lock().await.insert(
                answer.get_domain().to_string(),
                answer.clone(),
                Duration::from_secs(answer.get_ttl() as u64),
            );
        }

        Ok(packet)
    }
}

async fn handle_request(
    socket: Arc<UdpSocket>,
    mut req_buffer: BytePacketBuffer,
    src: SocketAddr,
) -> Result<()> {
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
        if let Ok(result) = lookup(&question.name, question.qtype, None).await {
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

    socket.send_to(data, src).await?;

    Ok(())
}

/// Handle a single incoming packet
async fn handle_query(socket: Arc<UdpSocket>) -> Result<()> {
    // With a socket ready, we can go ahead and read a packet. This will
    // block until one is received.
    let mut req_buffer = BytePacketBuffer::new();

    // The `recv_from` function will write the data into the provided buffer,
    // and return the length of the data read as well as the source address.
    // We're not interested in the length, but we need to keep track of the
    // source in order to send our reply later on.
    let (_, src) = socket.recv_from(&mut req_buffer.buf).await?;

    tokio::spawn(async move {
        match handle_request(socket, req_buffer, src).await {
            Ok(_) => {}
            Err(e) => eprintln!("An error occurred: {}", e),
        }
    });

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    get_blacklist();
    let socket = Arc::new(UdpSocket::bind(("127.0.0.1", PORT)).await?);
    println!("XDNS listening on port {}", PORT);

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            let mut cache = CACHE.lock().await;
            let cache_before_size = cache.size();
            cache.cleanup().await;
            let cache_after_size = cache.size();
            drop(cache);

            if cache_before_size != cache_after_size {
                println!(
                    "Cache cleanup removed {} entries",
                    cache_before_size - cache_after_size
                );
            }
        }
    });

    loop {
        let socket = socket.clone();
        match handle_query(socket).await {
            Ok(_) => {}
            Err(e) => eprintln!("An error occurred: {}", e),
        }
    }
}
