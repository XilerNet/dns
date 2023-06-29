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
        assert!(parsed.signature.expect("Signature should not be null?").is_valid(credentials));
    }};
}

#[test]
fn parse_action_domain_registration() {
    let input = "DOMAIN example.o 1685954907 null";
    parse_action!(input, DomainAction::Domain(_));
}

#[test]
fn parse_action_subdomain() {
    let input = "DNS example.o example. CNAME IN 30 example.com 987A990BE5A7C872E404321EBDA1D8B35B5159F65E349F8D2BAD76D716F5EF1C3724748B2724B8983C2F33B9D0E117F6B2B712B39EF18C4CB492488D88961E0A";
    parse_action_with_signature!(input, DomainAction::Subdomain(_));
}

#[test]
fn parse_action_drop() {
    let input = "DROP 1234567890 55590C3FE2BA102F5C3A530A25D40C3D08C1C56359AB92E14E9C308400294D1B23CB4D8E2CCCF7D4D7D49095789472ADA3854B46769A4F367A62010B00ACCF0C";
    parse_action_with_signature!(input, DomainAction::Drop(_));
}

#[test]
fn parse_action_validity() {
    let input = "DOMAIN-VALIDITY example.o ed25519 naRG4n_qit1jAPO5F1zJ-J7wPa2Dy8K-GOxhCu-9DDo null";
    parse_action!(input, DomainAction::Validity(_));
}

#[test]
fn parse_action_validity_transfer_without_new() {
    let input = "DOMAIN-VALIDATE-TRANSFER example.o A77265C64C44DE3DCFA2942C5B8944B5969484A9D69F4008AE3241472F6AC835BACC169A0B6A739DF447EC8B06DA38D1D531680CCE8B05A8E6734E1B1D72F90E";
    parse_action_with_signature!(input, DomainAction::ValidityTransfer(_));
}

#[test]
fn parse_action_validity_transfer_with_new() {
    let input = "DOMAIN-VALIDATE-TRANSFER example.o ed25519 naRG4n_qit1jAPO5F1zJ-J7wPa2Dy8K-GOxhCu-9DDo EE5C50FF961F148233205357E76010D6BBB3B10DDC8645D7021FF33ACB246534EBCFD8D3AF13A56D7CFB7CDB7251D0C06923C8E28E7E02B4FCB5324A7FE4AC03";
    parse_action_with_signature!(input, DomainAction::ValidityTransfer(_));
}

#[test]
fn parse_action_domain_data() {
    let input = "DOMAIN-DATA example.o Xiler 3238D1FD6C25CFFA4E4DD585AB83BEE66DCEE8528860E4B3BAE8E681042A6F4645DFD70CD15A6DCBDD245C0CD4EC374C294B97CC076B1AC366669EB32DFCB505";
    parse_action_with_signature!(input, DomainAction::Data(_));
}

#[test]
fn parse_action_invalid() {
    let input = "Some action content here which is not a domain data record!";
    let parsed = ActionParser::parse(input);

    assert!(parsed.is_err());
}