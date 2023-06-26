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
    assert_eq!(parsed_signature.content.len(), 1);
    assert_eq!(
        parsed_signature.content.first().unwrap(),
        "Xiler - decentralising the centralised"
    );
}

#[test]
fn parse_message_signature_with_new_line() {
    let signature = "6A4E49AA279FE3AF06A5A42B189CE179023FE51EEDEE67131AD855339E6216008270290D89459ED65991C8F4E799C82E558C94ED5A8B95B7FDEA61E5844B2F0C";
    let input = format!("Xiler - decentralising the centralised {}\n", signature);

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
        F6C92D6270FAFAA149EC229493056F80422001C26F442DCB906C3F7C5C75317DFE9DEC03AAA0297AB75C9FE9187BCC954A0F8DDA57E5E6A14424F00320AF7604
    "#;

    let parsed_signature = Signature::parse(&input);

    assert!(parsed_signature.is_ok());

    let parsed_signature = parsed_signature.unwrap();

    assert_eq!(parsed_signature.signature, "F6C92D6270FAFAA149EC229493056F80422001C26F442DCB906C3F7C5C75317DFE9DEC03AAA0297AB75C9FE9187BCC954A0F8DDA57E5E6A14424F00320AF7604");
    assert_eq!(parsed_signature.content.len(), 2);
    assert_eq!(parsed_signature.content[0], "DOMAIN example.o 1685954907");
    assert_eq!(parsed_signature.content[1], "DOMAIN-VALIDITY example.o 8AD5A3FFB6FFFA0E85F8C4D84A88B20FF2D2A430B0DBF73FEDB892B9EA45B1D6");
}

#[test]
fn parse_multiline_signature_after_last_content() {
    let input = r#"
        DOMAIN example.o 1685954907
        DOMAIN-VALIDITY example.o 8AD5A3FFB6FFFA0E85F8C4D84A88B20FF2D2A430B0DBF73FEDB892B9EA45B1D6 F6C92D6270FAFAA149EC229493056F80422001C26F442DCB906C3F7C5C75317DFE9DEC03AAA0297AB75C9FE9187BCC954A0F8DDA57E5E6A14424F00320AF7604
    "#;

    let parsed_signature = Signature::parse(&input);

    assert!(parsed_signature.is_ok());

    let parsed_signature = parsed_signature.unwrap();

    assert_eq!(parsed_signature.signature, "F6C92D6270FAFAA149EC229493056F80422001C26F442DCB906C3F7C5C75317DFE9DEC03AAA0297AB75C9FE9187BCC954A0F8DDA57E5E6A14424F00320AF7604");
    assert_eq!(parsed_signature.content.len(), 2);
    assert_eq!(parsed_signature.content[0], "DOMAIN example.o 1685954907");
    assert_eq!(parsed_signature.content[1], "DOMAIN-VALIDITY example.o 8AD5A3FFB6FFFA0E85F8C4D84A88B20FF2D2A430B0DBF73FEDB892B9EA45B1D6");
}

#[test]
fn parse_multiline_one_record() {
    let input = r#"
        DNS example.o e. CNAME IN 30 example.com 6fb976ab49dcec017f1e201e84395983204ae1a7c2abf7ced0a85d692e442799i0
        F6C92D6270FAFAA149EC229493056F80422001C26F442DCB906C3F7C5C75317DFE9DEC03AAA0297AB75C9FE9187BCC954A0F8DDA57E5E6A14424F00320AF7604
    "#;

    let parsed_signature = Signature::parse(&input);

    assert!(parsed_signature.is_ok());

    let parsed_signature = parsed_signature.unwrap();

    assert_eq!(parsed_signature.signature, "F6C92D6270FAFAA149EC229493056F80422001C26F442DCB906C3F7C5C75317DFE9DEC03AAA0297AB75C9FE9187BCC954A0F8DDA57E5E6A14424F00320AF7604");
    assert_eq!(parsed_signature.content.len(), 1);
    assert_eq!(parsed_signature.content[0], "DNS example.o e. CNAME IN 30 example.com 6fb976ab49dcec017f1e201e84395983204ae1a7c2abf7ced0a85d692e442799i0");
}
