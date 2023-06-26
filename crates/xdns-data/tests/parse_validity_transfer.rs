use xdns_data::models::algorithm::Algorithm;
use xdns_data::models::validity_transfer::ValidityTransfer;
use xdns_data::prelude::Parser;

#[test]
fn parse_valid_transfer_without_new() {
    let input = "DOMAIN-VALIDATE-TRANSFER example.o";
    let parsed = ValidityTransfer::parse(input);

    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();
    assert_eq!(parsed.domain, "example.o");
    assert!(matches!(parsed.new_credentials, None));
}

#[test]
fn parse_valid_transfer_with_new() {
    let input =
        "DOMAIN-VALIDATE-TRANSFER example.o ed25519 naRG4n_qit1jAPO5F1zJ-J7wPa2Dy8K-GOxhCu-9DDo";
    let parsed = ValidityTransfer::parse(input);

    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();
    assert_eq!(parsed.domain, "example.o");
    assert!(matches!(parsed.new_credentials, Some(_)));

    let credentials = parsed.new_credentials.unwrap();
    assert_eq!(credentials.algorithm, Algorithm::Ed25519);
    assert_eq!(
        credentials.public_key,
        "naRG4n_qit1jAPO5F1zJ-J7wPa2Dy8K-GOxhCu-9DDo"
    );
}

#[test]
fn parse_invalid_transfer_input() {
    let input = "Some inscription content here which is not a domain validity transfer record!";
    let parsed = ValidityTransfer::parse(input);

    assert!(parsed.is_err());
}

#[test]
fn parse_invalid_transfer_domain() {
    let input = "DOMAIN-VALIDATE-TRANSFER invalid";
    let parsed = ValidityTransfer::parse(input);

    assert!(parsed.is_err());
}

#[test]
fn parse_invalid_transfer_algorithm() {
    let input =
        "DOMAIN-VALIDATE-TRANSFER example.o invalid naRG4n_qit1jAPO5F1zJ-J7wPa2Dy8K-GOxhCu-9DDo";
    let parsed = ValidityTransfer::parse(input);

    assert!(parsed.is_err());
}

#[test]
fn parse_invalid_transfer_missing_key() {
    let input = "DOMAIN-VALIDATE-TRANSFER example.o ed25519";
    let parsed = ValidityTransfer::parse(input);

    assert!(parsed.is_err());
}

#[test]
fn parse_invalid_transfer_key() {
    let input = "DOMAIN-VALIDATE-TRANSFER example.o ed25519 invalid key (mustn't contain spaces)";
    let parsed = ValidityTransfer::parse(input);

    assert!(parsed.is_err());
}
