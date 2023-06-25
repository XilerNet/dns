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

#[test]
pub fn is_domain_name_valid_normal() {
    let domain = "example.o";
    let is_valid = Domain::is_valid_domain_name(domain);

    assert!(is_valid);
}

#[test]
pub fn is_domain_name_valid_one_character() {
    let domain = "e.o";
    let is_valid = Domain::is_valid_domain_name(domain);

    assert!(is_valid);
}

#[test]
pub fn is_domain_name_valid_hyphen() {
    let domain = "my-example.o";
    let is_valid = Domain::is_valid_domain_name(domain);

    assert!(is_valid);
}

#[test]
pub fn is_domain_name_valid_numeric() {
    let domain = "54t05h1.o";
    let is_valid = Domain::is_valid_domain_name(domain);

    assert!(is_valid);
}

#[test]
pub fn is_domain_name_invalid_empty() {
    let domain = ".o";
    let is_valid = Domain::is_valid_domain_name(domain);

    assert!(!is_valid);
}

#[test]
pub fn is_domain_name_invalid_too_long() {
    let domain = "111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111.o";
    let is_valid = Domain::is_valid_domain_name(domain);

    assert!(!is_valid);
}

#[test]
pub fn is_domain_name_invalid_prefix_hyphen() {
    let domain = "-invalid.o";
    let is_valid = Domain::is_valid_domain_name(domain);

    assert!(!is_valid);
}

#[test]
pub fn is_domain_name_invalid_character() {
    let domain = "invalid*.o";
    let is_valid = Domain::is_valid_domain_name(domain);

    assert!(!is_valid);
}

#[test]
pub fn is_domain_name_invalid_single_character() {
    let domain = "*.o";
    let is_valid = Domain::is_valid_domain_name(domain);

    assert!(!is_valid);
}

#[test]
pub fn is_domain_name_invalid_uppercase() {
    let domain = "INVALID.o";
    let is_valid = Domain::is_valid_domain_name(domain);

    assert!(!is_valid);
}

#[test]
pub fn is_domain_name_invalid_missing_tld() {
    let domain = "invalid";
    let is_valid = Domain::is_valid_domain_name(domain);

    assert!(!is_valid);
}

#[test]
pub fn is_valid_character_normal() {
    let character = 'a';
    let is_valid = Domain::is_valid_character(character);

    assert!(is_valid);
}

#[test]
pub fn is_valid_character_numeric() {
    let character = '1';
    let is_valid = Domain::is_valid_character(character);

    assert!(is_valid);
}

#[test]
pub fn is_valid_character_hyphen() {
    let character = '-';
    let is_valid = Domain::is_valid_character(character);

    assert!(is_valid);
}

#[test]
pub fn is_invalid_character() {
    let character = '*';
    let is_valid = Domain::is_valid_character(character);

    assert!(!is_valid);
}

#[test]
pub fn is_invalid_character_uppercase() {
    let character = 'A';
    let is_valid = Domain::is_valid_character(character);

    assert!(!is_valid);
}

#[test]
pub fn is_valid_edge_character_normal() {
    let character = 'a' as u8;
    let is_valid = Domain::is_valid_edge_character(character);

    assert!(is_valid);
}

#[test]
pub fn is_valid_edge_character_numeric() {
    let character = '1' as u8;
    let is_valid = Domain::is_valid_edge_character(character);

    assert!(is_valid);
}

#[test]
pub fn is_invalid_edge_character_hyphen() {
    let character = '-' as u8;
    let is_valid = Domain::is_valid_edge_character(character);

    assert!(!is_valid);
}

#[test]
pub fn is_invalid_edge_character() {
    let character = '*' as u8;
    let is_valid = Domain::is_valid_edge_character(character);

    assert!(!is_valid);
}

#[test]
pub fn is_invalid_edge_character_uppercase() {
    let character = 'A' as u8;
    let is_valid = Domain::is_valid_edge_character(character);

    assert!(!is_valid);
}