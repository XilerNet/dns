use xdns_data::models::validity::{Algorithm, Validity};

#[test]
fn valid_validity() {
    let input = "DOMAIN-VALIDITY example.o ed25519 naRG4n_qit1jAPO5F1zJ-J7wPa2Dy8K-GOxhCu-9DDo";
    let validity = Validity::parse(input);

    assert!(validity.is_ok());

    let validity = validity.unwrap();
    assert_eq!(validity.domain, "example.o");
    assert_eq!(validity.algorithm, Algorithm::Ed25519);
    assert_eq!(validity.key, "naRG4n_qit1jAPO5F1zJ-J7wPa2Dy8K-GOxhCu-9DDo".as_bytes().to_vec());
}

#[test]
fn invalid_domain() {
    let input = "DOMAIN-VALIDITY invalid ed25519 naRG4n_qit1jAPO5F1zJ-J7wPa2Dy8K-GOxhCu-9DDo";
    let validity = Validity::parse(input);

    assert!(validity.is_err());
}

#[test]
fn invalid_algorithm() {
    let input = "DOMAIN-VALIDITY example.o invalid naRG4n_qit1jAPO5F1zJ-J7wPa2Dy8K-GOxhCu-9DDo";
    let validity = Validity::parse(input);

    assert!(validity.is_err());
}

#[test]
fn invalid_missing_key() {
    let input = "DOMAIN-VALIDITY example.o ed25519";
    let validity = Validity::parse(input);

    assert!(validity.is_err());
}

#[test]
fn invalid_key() {
    let input = "DOMAIN-VALIDITY example.o ed25519 invalid key (mustn't contain spaces)";
    let validity = Validity::parse(input);

    assert!(validity.is_err());
}