use xdns_data::models::subdomain::SubDomain;

#[test]
fn parse_subdomain_valid() {
    let input = "DNS example.o example. CNAME IN 30 example.com";
    let parsed = SubDomain::parse(input);

    assert!(parsed.is_ok());
}

#[test]
fn parse_subdomain_valid_one_character() {
    let input = "DNS example.o e. CNAME IN 30 example.com";
    let parsed = SubDomain::parse(input);

    assert!(parsed.is_ok());
}

#[test]
fn parse_subdomain_valid_root() {
    let input = "DNS example.o . CNAME IN 30 example.com";
    let parsed = SubDomain::parse(input);

    assert!(parsed.is_ok());
}

#[test]
fn parse_subdomain_valid_other_type() {
    let input = "DNS example.o example. A IN 30 127.0.0.1";
    let parsed = SubDomain::parse(input);

    assert!(parsed.is_ok());
}

#[test]
fn parse_subdomain_valid_big_ttl() {
    let input = "DNS example.o example. A IN 400000000 127.0.0.1";
    let parsed = SubDomain::parse(input);

    assert!(parsed.is_ok());
}

#[test]
fn parse_subdomain_valid_depth() {
    let input = "DNS example.o example.other. A IN 30 127.0.0.1";
    let parsed = SubDomain::parse(input);

    assert!(parsed.is_ok());
}

#[test]
fn parse_subdomain_valid_hypen() {
    let input = "DNS example.o example-other. A IN 30 127.0.0.1";
    let parsed = SubDomain::parse(input);

    assert!(parsed.is_ok());
}

#[test]
fn parse_subdomain_invalid_domain() {
    let input = "DNS invalid example-other. A IN 30 127.0.0.1";
    let parsed = SubDomain::parse(input);

    assert!(parsed.is_err());
}

#[test]
fn parse_subdomain_invalid_missing_suffix_dot() {
    let input = "DNS invalid.o example CNAME IN 30 example.com";
    let parsed = SubDomain::parse(input);

    assert!(parsed.is_err());
}

#[test]
fn parse_subdomain_invalid_invalid_suffix_hyphen() {
    let input = "DNS invalid.o example-. CNAME IN 30 example.com";
    let parsed = SubDomain::parse(input);

    assert!(parsed.is_err());
}

#[test]
fn parse_subdomain_invalid_invalid_prefix_hyphen() {
    let input = "DNS invalid.o -example. CNAME IN 30 example.com";
    let parsed = SubDomain::parse(input);

    assert!(parsed.is_err());
}

#[test]
fn parse_subdomain_invalid_invalid_suffix_double_dot() {
    let input = "DNS invalid.o example.. CNAME IN 30 example.com";
    let parsed = SubDomain::parse(input);

    assert!(parsed.is_err());
}

#[test]
fn parse_subdomain_invalid_invalid_double_dot() {
    let input = "DNS invalid.o my..example. CNAME IN 30 example.com";
    let parsed = SubDomain::parse(input);

    assert!(parsed.is_err());
}

#[test]
fn parse_subdomain_invalid_invalid_prefix_dot() {
    let input = "DNS invalid.o .example. CNAME IN 30 example.com";
    let parsed = SubDomain::parse(input);

    assert!(parsed.is_err());
}

#[test]
fn parse_subdomain_invalid_invalid_uppercase() {
    let input = "DNS invalid.o EXAMPLE. CNAME IN 30 example.com";
    let parsed = SubDomain::parse(input);

    assert!(parsed.is_err());
}

#[test]
fn parse_subdomain_invalid_invalid_ttl() {
    let input = "DNS invalid.o example. A IN 99999999999 127.0.0.1";
    let parsed = SubDomain::parse(input);

    assert!(parsed.is_err());
}

#[test]
fn parse_subdomain_invalid_invalid_ttl_negative() {
    let input = "DNS invalid.o example. A IN -1 127.0.0.1";
    let parsed = SubDomain::parse(input);

    assert!(parsed.is_err());
}

#[test]
fn parse_subdomain_invalid_dash_dot_neighbor_pre() {
    let input = "DNS invalid.o my-.example.o CNAME IN 30 example.com";
    let parsed = SubDomain::parse(input);

    assert!(parsed.is_err());
}

#[test]
fn parse_subdomain_invalid_dash_dot_neighbor_sub() {
    let input = "DNS invalid.o my.-example.o CNAME IN 30 example.com";
    let parsed = SubDomain::parse(input);

    assert!(parsed.is_err());
}

#[test]
fn valid_subdomain_character() {
    let character = 'a';
    let valid = SubDomain::is_valid_character(character);

    assert!(valid);
}

#[test]
fn valid_subdomain_character_number() {
    let character = '1';
    let valid = SubDomain::is_valid_character(character);

    assert!(valid);
}

#[test]
fn valid_subdomain_character_hyphen() {
    let character = '-';
    let valid = SubDomain::is_valid_character(character);

    assert!(valid);
}

#[test]
fn valid_subdomain_character_dot() {
    let character = '.';
    let valid = SubDomain::is_valid_character(character);

    assert!(valid);
}

#[test]
fn valid_subdomain_character_astrix() {
    let character = '*';
    let valid = SubDomain::is_valid_character(character);

    assert!(valid);
}

#[test]
fn invalid_subdomain_character_uppercase() {
    let character = 'A';
    let valid = SubDomain::is_valid_character(character);

    assert!(!valid);
}

#[test]
fn valid_subdomain() {
    let subdomain = "example.";
    let valid = SubDomain::is_valid_subdomain(subdomain);

    assert!(valid);
}

#[test]
fn valid_subdomain_hyphen() {
    let subdomain = "example-other.";
    let valid = SubDomain::is_valid_subdomain(subdomain);

    assert!(valid);
}

#[test]
fn valid_subdomain_wildcard() {
    let subdomain = "*.example.";
    let valid = SubDomain::is_valid_subdomain(subdomain);

    assert!(valid);
}

#[test]
fn valid_subdomain_numbers() {
    let subdomain = "123.";
    let valid = SubDomain::is_valid_subdomain(subdomain);

    assert!(valid);
}

#[test]
fn valid_depth_subdomain() {
    let subdomain = "example.other.";
    let valid = SubDomain::is_valid_subdomain(subdomain);

    assert!(valid);
}

#[test]
fn valid_depth_subdomain_numbers() {
    let subdomain = "123.456.";
    let valid = SubDomain::is_valid_subdomain(subdomain);

    assert!(valid);
}

#[test]
fn invalid_subdomain_dot_start() {
    let subdomain = ".example-other.";
    let valid = SubDomain::is_valid_subdomain(subdomain);

    assert!(!valid);
}

#[test]
fn invalid_subdomain_no_dot_end() {
    let subdomain = "example-other";
    let valid = SubDomain::is_valid_subdomain(subdomain);

    assert!(!valid);
}

#[test]
fn invalid_subdomain_hyphen_start() {
    let subdomain = "-example-other.";
    let valid = SubDomain::is_valid_subdomain(subdomain);

    assert!(!valid);
}

#[test]
fn invalid_subdomain_hyphen_end() {
    let subdomain = "example-other-.";
    let valid = SubDomain::is_valid_subdomain(subdomain);

    assert!(!valid);
}

#[test]
fn invalid_subdomain_pre_adjacent_hypen_to_dot() {
    let subdomain = "example-.other.";
    let valid = SubDomain::is_valid_subdomain(subdomain);

    assert!(!valid);
}

#[test]
fn invalid_subdomain_adjacent_hypen_to_dot() {
    let subdomain = "example.-other.";
    let valid = SubDomain::is_valid_subdomain(subdomain);

    assert!(!valid);
}
