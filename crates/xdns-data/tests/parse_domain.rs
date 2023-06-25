use shared::time::system_time_from_epoch_seconds;
use xdns_data::models::domain::Domain;

#[test]
pub fn parse_domain_normal() {
    let domain = "DOMAIN example.o 1685954907";
    let parsed = Domain::parse(domain);

    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();

    assert_eq!(parsed.name, "example.o");
    assert_eq!(parsed.valid_from, system_time_from_epoch_seconds(1685954907));
}

#[test]
pub fn parse_domain_one_character() {
    let domain = "DOMAIN e.o 1685954907";
    let parsed = Domain::parse(domain);

    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();

    assert_eq!(parsed.name, "e.o");
    assert_eq!(parsed.valid_from, system_time_from_epoch_seconds(1685954907));
}

#[test]
pub fn parse_domain_hyphen() {
    let domain = "DOMAIN my-example.o 1685954907";
    let parsed = Domain::parse(domain);

    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();

    assert_eq!(parsed.name, "my-example.o");
    assert_eq!(parsed.valid_from, system_time_from_epoch_seconds(1685954907));
}

#[test]
pub fn parse_domain_numeric() {
    let domain = "DOMAIN 54t05h1.o 1685954907";
    let parsed = Domain::parse(domain);

    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();

    assert_eq!(parsed.name, "54t05h1.o");
    assert_eq!(parsed.valid_from, system_time_from_epoch_seconds(1685954907));
}

#[test]
pub fn parse_domain_invalid_empty() {
    let domain = "DOMAIN .o 1685954907";
    let parsed = Domain::parse(domain);

    assert!(parsed.is_err());
}

#[test]
pub fn parse_domain_invalid_too_long() {
    let domain = "DOMAIN 111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111.o 1685954907";
    let parsed = Domain::parse(domain);

    assert!(parsed.is_err());
}

#[test]
pub fn parse_domain_invalid_prefix_hyphen() {
    let domain = "DOMAIN -invalid.o 1685954907";
    let parsed = Domain::parse(domain);

    assert!(parsed.is_err());
}

#[test]
pub fn parse_domain_invalid_character() {
    let domain = "DOMAIN invalid*.o 1685954907";
    let parsed = Domain::parse(domain);

    assert!(parsed.is_err());
}

#[test]
pub fn parse_domain_invalid_single_character() {
    let domain = "DOMAIN *.o 1685954907";
    let parsed = Domain::parse(domain);

    assert!(parsed.is_err());
}

#[test]
pub fn parse_domain_invalid_uppercase() {
    let domain = "DOMAIN INVALID.o 1685954907";
    let parsed = Domain::parse(domain);

    assert!(parsed.is_err());
}

#[test]
pub fn parse_domain_invalid_missing_tld() {
    let domain = "DOMAIN invalid 1685954907";
    let parsed = Domain::parse(domain);

    assert!(parsed.is_err());
}

#[test]
pub fn parse_domain_invalid_missing_epoch() {
    let domain = "DOMAIN invalid.o";
    let parsed = Domain::parse(domain);

    assert!(parsed.is_err());
}
