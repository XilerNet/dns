use xdns_data::models::data::Data;
use xdns_data::prelude::Parser;

#[test]
fn parse_valid_domain_data_without_spaces() {
    let input = "DOMAIN-DATA example.o Xiler";
    let parsed = Data::parse(input);

    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();
    assert_eq!(parsed.domain, "example.o");
    assert_eq!(parsed.data, b"Xiler");
}

#[test]
fn parse_valid_domain_data_with_spaces() {
    let input = "DOMAIN-DATA example.o Xiler - decentralising the centralised";
    let parsed = Data::parse(input);

    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();
    assert_eq!(parsed.domain, "example.o");
    assert_eq!(parsed.data, b"Xiler - decentralising the centralised");
}

#[test]
fn parse_invalid_domain_data_domain() {
    let input = "DOMAIN-DATA invalid Xiler";
    let parsed = Data::parse(input);

    assert!(parsed.is_err());
}

#[test]
fn parse_invalid_domain_data() {
    let input = "DOMAIN-DATA example.o";
    let parsed = Data::parse(input);

    assert!(parsed.is_err());
}

#[test]
fn parse_invalid_input() {
    let input = "Some inscription content here which is not a domain data record!";
    let parsed = Data::parse(input);

    assert!(parsed.is_err());
}
