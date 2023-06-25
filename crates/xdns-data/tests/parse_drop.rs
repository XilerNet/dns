use xdns_data::models::drop::DomainDrop;
use xdns_data::prelude::Parser;

#[test]
fn parse_drop() {
    let input = "DROP 1234567890";
    let drop = DomainDrop::parse(input);

    assert!(drop.is_ok());
}

#[test]
fn parse_invalid_drop() {
    let input = "DROP";
    let drop = DomainDrop::parse(input);

    assert!(drop.is_err());
}

#[test]
fn parse_invalid_drop_with_spaces() {
    let input = "DROP  ";
    let drop = DomainDrop::parse(input);

    assert!(drop.is_err());
}

#[test]
fn parse_invalid_input() {
    let domain = "Some inscription content here which is not a drop record!";
    let parsed = DomainDrop::parse(domain);

    assert!(parsed.is_err());
}
