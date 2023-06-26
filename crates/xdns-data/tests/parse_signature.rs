use xdns_data::models::signature::Signature;
use xdns_data::prelude::Parser;

#[test]
fn parse_message_signature() {
    let signature = "6A4E49AA279FE3AF06A5A42B189CE179023FE51EEDEE67131AD855339E6216008270290D89459ED65991C8F4E799C82E558C94ED5A8B95B7FDEA61E5844B2F0C";
    let input = format!("Xiler - decentralising the centralised {}", signature);

    let parsed_signature = Signature::parse(&input);
    assert!(parsed_signature.is_ok());

    let parsed_signature = parsed_signature.unwrap();
    assert_eq!(parsed_signature.signature, signature);
    assert_eq!(parsed_signature.content, "Xiler - decentralising the centralised");
}

#[test]
fn parse_empty_string() {
    let input = "";

    let parsed_signature = Signature::parse(&input);
    assert!(parsed_signature.is_err());
}

#[test]
fn parse_invalid_signature() {
    let input = "invalid signature";

    let parsed_signature = Signature::parse(&input);
    assert!(parsed_signature.is_err());
}