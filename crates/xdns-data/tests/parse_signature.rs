use xdns_data::models::signature::Signature;
use xdns_data::models::{Algorithm, Credentials};
use xdns_data::prelude::Parser;

const PUBLIC_KEY: &'static str = "C0AB4030035B8DDA5E9F5BF3881B8E21603714674AF8099602F31F142D80BCFE";

#[test]
fn parse_message_signature() {
    let signature = "1E4204ECC1693A7CEC3CF05E3E8E4C3E32FB046C895A3ED9A687DD0561BD93C89DF2EA6EC1FBACE2ED16614DFCB1C741CF2C704285805C5ED3B866F5208FDD0C";
    let input = format!("Xiler - decentralising the centralised null {}", signature);

    let parsed_signature = Signature::parse(&input);
    assert!(parsed_signature.is_ok());

    let parsed_signature = parsed_signature.unwrap();
    assert_eq!(parsed_signature.signature, signature);
    assert_eq!(parsed_signature.content.len(), 1);
    assert_eq!(
        parsed_signature.content.first().unwrap(),
        "Xiler - decentralising the centralised"
    );
    let credentials = Credentials::new(Algorithm::Ed25519, PUBLIC_KEY.to_string());
    assert!(parsed_signature.is_valid(credentials));
}

#[test]
fn parse_message_signature_with_new_line() {
    let signature = "46227E3625F8786DF951A53E3EE4BF85084D1FEC0C4D6376644740281554DB4A1A67BB8F1EF699F8AAFE7151D42BDBF9EE92DAD8B84030F5CE6FE171D1C0370D";
    let input = format!(
        "Xiler - decentralising the centralised null {}\n",
        signature
    );

    let parsed_signature = Signature::parse(&input);
    assert!(parsed_signature.is_ok());

    let parsed_signature = parsed_signature.unwrap();
    assert_eq!(parsed_signature.signature, signature);
    assert_eq!(parsed_signature.content.len(), 1);
    assert_eq!(
        parsed_signature.content.first().unwrap(),
        "Xiler - decentralising the centralised"
    );
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

#[test]
fn parse_multiline_signature() {
    let input = r#"
        DOMAIN example.o 1685954907
        DOMAIN-VALIDITY example.o 8AD5A3FFB6FFFA0E85F8C4D84A88B20FF2D2A430B0DBF73FEDB892B9EA45B1D6
        null B6E4C817D5A65040A7514B60D0FEABA340868B5D8AB84471E0AB25CE5E1089329CDF983A698381BBC9E2FF5DF11EC06F2D9020123A0C10D95ACB84FB0CA03F0F
    "#;

    let parsed_signature = Signature::parse(&input);

    assert!(parsed_signature.is_ok());

    let parsed_signature = parsed_signature.unwrap();

    assert_eq!(parsed_signature.signature, "B6E4C817D5A65040A7514B60D0FEABA340868B5D8AB84471E0AB25CE5E1089329CDF983A698381BBC9E2FF5DF11EC06F2D9020123A0C10D95ACB84FB0CA03F0F");
    assert_eq!(parsed_signature.last_id, None);
    assert_eq!(parsed_signature.content.len(), 2);
    assert_eq!(parsed_signature.content[0], "DOMAIN example.o 1685954907");
    assert_eq!(parsed_signature.content[1], "DOMAIN-VALIDITY example.o 8AD5A3FFB6FFFA0E85F8C4D84A88B20FF2D2A430B0DBF73FEDB892B9EA45B1D6");

    let credentials = Credentials::new(Algorithm::Ed25519, PUBLIC_KEY.to_string());
    assert!(parsed_signature.is_valid(credentials));
}

#[test]
fn parse_multiline_signature_after_last_content() {
    let input = r#"
        DOMAIN example.o 1685954907
        DOMAIN-VALIDITY example.o 8AD5A3FFB6FFFA0E85F8C4D84A88B20FF2D2A430B0DBF73FEDB892B9EA45B1D6 null F6C92D6270FAFAA149EC229493056F80422001C26F442DCB906C3F7C5C75317DFE9DEC03AAA0297AB75C9FE9187BCC954A0F8DDA57E5E6A14424F00320AF7604
    "#;

    let parsed_signature = Signature::parse(&input);

    assert!(parsed_signature.is_ok());

    let parsed_signature = parsed_signature.unwrap();

    assert_eq!(parsed_signature.last_id, None);
    assert_eq!(parsed_signature.signature, "F6C92D6270FAFAA149EC229493056F80422001C26F442DCB906C3F7C5C75317DFE9DEC03AAA0297AB75C9FE9187BCC954A0F8DDA57E5E6A14424F00320AF7604");
    assert_eq!(parsed_signature.content.len(), 2);
    assert_eq!(parsed_signature.content[0], "DOMAIN example.o 1685954907");
    assert_eq!(parsed_signature.content[1], "DOMAIN-VALIDITY example.o 8AD5A3FFB6FFFA0E85F8C4D84A88B20FF2D2A430B0DBF73FEDB892B9EA45B1D6");
}

#[test]
fn parse_multiline_one_record() {
    let input = r#"
        DNS example.o e. CNAME IN 30 example.com 6fb976ab49dcec017f1e201e84395983204ae1a7c2abf7ced0a85d692e442799i0
        null F6C92D6270FAFAA149EC229493056F80422001C26F442DCB906C3F7C5C75317DFE9DEC03AAA0297AB75C9FE9187BCC954A0F8DDA57E5E6A14424F00320AF7604
    "#;

    let parsed_signature = Signature::parse(&input);

    assert!(parsed_signature.is_ok());

    let parsed_signature = parsed_signature.unwrap();

    assert_eq!(parsed_signature.last_id, None);
    assert_eq!(parsed_signature.signature, "F6C92D6270FAFAA149EC229493056F80422001C26F442DCB906C3F7C5C75317DFE9DEC03AAA0297AB75C9FE9187BCC954A0F8DDA57E5E6A14424F00320AF7604");
    assert_eq!(parsed_signature.content.len(), 1);
    assert_eq!(parsed_signature.content[0], "DNS example.o e. CNAME IN 30 example.com 6fb976ab49dcec017f1e201e84395983204ae1a7c2abf7ced0a85d692e442799i0");
}

#[test]
fn parse_multiline_with_last_id() {
    let input = r#"
        DOMAIN example.o 1685954907
        DOMAIN-VALIDITY example.o 8AD5A3FFB6FFFA0E85F8C4D84A88B20FF2D2A430B0DBF73FEDB892B9EA45B1D6
        hello null
    "#;

    let parsed_signature = Signature::parse(&input);

    assert!(parsed_signature.is_ok());

    let parsed_signature = parsed_signature.unwrap();

    assert_eq!(parsed_signature.last_id, Some("hello".to_string()));
}

#[test]
fn test_single_line_with_last_id() {
    let input = "My inscription content hello null";
    let parsed_signature = Signature::parse(&input);

    assert!(parsed_signature.is_ok());

    let parsed_signature = parsed_signature.unwrap();

    assert_eq!(parsed_signature.last_id, Some("hello".to_string()));
}
