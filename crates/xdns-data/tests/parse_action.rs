use xdns_data::models::algorithm::Algorithm;
use xdns_data::models::credentials::Credentials;
use xdns_data::parser::{ActionParser, DomainAction};
use xdns_data::traits::Parser;

// private key for adding tests: 8BC8BE4BB432DCABFFD48501B72E2CE6AA8B285EFC6048F23818DF1E1EB47689
const PUBLIC_KEY: &str = "C0AB4030035B8DDA5E9F5BF3881B8E21603714674AF8099602F31F142D80BCFE";

macro_rules! parse_action {
    ($input:expr, $pattern:pat) => {{
        let parsed = ActionParser::parse($input);
        assert!(parsed.is_ok());

        let parsed = parsed.unwrap();
        assert_eq!(parsed.actions.len(), 1);
        assert!(matches!(parsed.actions[0], $pattern));

        parsed
    }};
}

macro_rules! parse_action_with_signature {
    ($input:expr, $pattern:pat) => {{
        let parsed = parse_action!($input, $pattern);
        let credentials = Credentials::new(Algorithm::Ed25519, PUBLIC_KEY.to_string());
        assert!(parsed
            .signature
            .expect("Signature should not be null?")
            .is_valid(credentials));
    }};
}

#[test]
fn parse_action_domain_registration() {
    let input = "DOMAIN example.o 1685954907 null null";
    parse_action!(input, DomainAction::Domain(_));
}

#[test]
fn parse_action_subdomain() {
    let input = "DNS example.o example. CNAME IN 30 example.com null 92C7B915886F7B854A4DED977BFCA54742925D59B0C0D4382B17D7B6FF18C0EFB3AC773965A2BD60A70299D730C5306587A72578F4D3FA944594AC777C9A3B07";
    parse_action_with_signature!(input, DomainAction::Subdomain(_));
}

#[test]
fn parse_action_drop() {
    let input = "DROP 1234567890 null DCD7212911B444837FD1C53917BA8A6ABDCE3E1A3FA44049237089DD87CA7AA696494FA2A61E704F5B71BDAB81633188BAB6171A245240C2957E2DD1E4D8A40E";
    parse_action_with_signature!(input, DomainAction::Drop(_));
}

#[test]
fn parse_action_validity() {
    let input =
        "DOMAIN-VALIDITY example.o ed25519 null naRG4n_qit1jAPO5F1zJ-J7wPa2Dy8K-GOxhCu-9DDo null";
    parse_action!(input, DomainAction::Validity(_));
}

#[test]
fn parse_action_validity_transfer_without_new() {
    let input = "DOMAIN-VALIDATE-TRANSFER example.o null 170A1DE6B4C476D6FF70E6091FAD50EF054E817BDA3C74D35390192901400B0C8F439FF766A53B673D45DAAD1266F3B63F1614D2D3FA5EEE8B5E7D334F56AC0F";
    parse_action_with_signature!(input, DomainAction::ValidityTransfer(_));
}

#[test]
fn parse_action_validity_transfer_with_new() {
    let input = "DOMAIN-VALIDATE-TRANSFER example.o ed25519 naRG4n_qit1jAPO5F1zJ-J7wPa2Dy8K-GOxhCu-9DDo null F04777CDADDE6251ECAC99DEDCC01B677056FC62A96B9C03A88B5E520BA4025F837F8CD6AA16F9605661BB95C85ECBC9AAC5003A083C8FA4DEE2B80971704405";
    parse_action_with_signature!(input, DomainAction::ValidityTransfer(_));
}

#[test]
fn parse_action_domain_data() {
    let input = "DOMAIN-DATA example.o Xiler null B94932262AA3BC6B30CCB8AFF24E7A2DAC8F923AF8988CE00959A934F4670AF4CAE77227F3E298C6AB9631AA2B7186DEDB25C7E837246C110FC3F45D67864203";
    parse_action_with_signature!(input, DomainAction::Data(_));
}

#[test]
fn parse_action_invalid() {
    let input = "Some action content here which is not a domain data record!";
    let parsed = ActionParser::parse(input);

    assert!(parsed.is_err());
}

#[test]
fn parse_multiple_actions() {
    let input = r#"
        DNS example.o 1. CNAME IN 30 example.com
        DNS example.o 2. CNAME IN 30 example.com
        DNS example.o 3. CNAME IN 30 example.com
        DNS example.o 4. CNAME IN 30 example.com
        6fb976ab49dcec017f1e201e84395983204ae1a7c2abf7ced0a85d692e442799i0 273EBA5F5D71E3A9DB5F273EACA418974073263FA53474817DB12FC0FCF9DE6266499BC66A155E4A9EABC20ACBB244BF349BE5276430DC4C5DF4A8E89FABA106
    "#;

    let parsed = ActionParser::parse(input);
    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();
    assert_eq!(parsed.actions.len(), 4);

    for action in parsed.actions {
        assert!(matches!(action, DomainAction::Subdomain(_)));
    }
}

#[test]
fn parse_domain_with_validity() {
    let input = r#"
        DOMAIN satoshi.o 1688852425
        DOMAIN-VALIDITY satoshi.o ed25519 66D8C046FD99D155338B40155D22E24229C1D4D897BC5B327414DFD8D0946D5E
        null null
    "#;

    let parsed = ActionParser::parse(input);
    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();
    assert_eq!(parsed.actions.len(), 2);

    assert!(matches!(parsed.actions[0], DomainAction::Domain(_)));
    assert!(matches!(parsed.actions[1], DomainAction::Validity(_)));
}

#[test]
fn parse_dns_records() {
    let input = r#"
        DNS satoshi.o @. CNAME IN 60 xiler.net
        DNS satoshi.o proxy. CNAME IN 60 dns-test-proxy-cname.xiler.net
        DNS satishi.o local. A IN 60 127.0.0.1
        null C0E53C29B279681E43989E46F4CD3269BF585F970F7EF94C222BC2148E712865C8707230C017455C6FC574838CF7EE585845A1B3EEC198FA50B01B5696039609
    "#;

    let parsed = ActionParser::parse(input);
    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();
    assert_eq!(parsed.actions.len(), 3);
}
