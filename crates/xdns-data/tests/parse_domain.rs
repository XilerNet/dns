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
pub fn parse_domain_invalid_keyword() {
    let domain = "DOMAINS invalid.o 1685954907";
    let parsed = Domain::parse(domain);

    assert!(parsed.is_err());
}

#[test]
pub fn parse_domain_invalid_keyword_lowercase() {
    let domain = "domain invalid.o 1685954907";
    let parsed = Domain::parse(domain);

    assert!(parsed.is_err());
}

#[test]
pub fn parse_domain_invalid_data() {
    let domain = "Some inscription content here which is not a domain record!";
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
    let character = 'a';
    let is_valid = Domain::is_valid_edge_character(character);

    assert!(is_valid);
}

#[test]
pub fn is_valid_edge_character_numeric() {
    let character = '1';
    let is_valid = Domain::is_valid_edge_character(character);

    assert!(is_valid);
}

#[test]
pub fn is_invalid_edge_character_hyphen() {
    let character = '-';
    let is_valid = Domain::is_valid_edge_character(character);

    assert!(!is_valid);
}

#[test]
pub fn is_invalid_edge_character() {
    let character = '*';
    let is_valid = Domain::is_valid_edge_character(character);

    assert!(!is_valid);
}

#[test]
pub fn is_invalid_edge_character_uppercase() {
    let character = 'A';
    let is_valid = Domain::is_valid_edge_character(character);

    assert!(!is_valid);
}

#[test]
pub fn get_tld_single_char() {
    let domain = "example.o";
    let tld = Domain::get_tld(domain);

    assert!(tld.is_some());
    assert_eq!(tld.unwrap(), "o");
}

#[test]
pub fn get_tld_multi_char() {
    let domain = "example.com";
    let tld = Domain::get_tld(domain);

    assert!(tld.is_some());
    assert_eq!(tld.unwrap(), "com");
}

#[test]
pub fn get_tld_none() {
    let domain = "example";
    let tld = Domain::get_tld(domain);

    assert!(tld.is_none());
}

#[test]
pub fn has_valid_o_tld() {
    let domain = "example.o";
    let has_o_tld = Domain::is_tld_valid(domain);

    assert!(has_o_tld);
}

#[test]
pub fn has_invalid_o_tld() {
    let domain = "example.com";
    let has_o_tld = Domain::is_tld_valid(domain);

    assert!(!has_o_tld);
}