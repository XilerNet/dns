use hex_literal::hex;
use xdns_data::models::algorithm::Algorithm;

static MESSAGE: &[u8] = b"Xiler - decentralising the centralised";

#[test]
fn ed25519() {
    let public_key = hex!("8AD5A3FFB6FFFA0E85F8C4D84A88B20FF2D2A430B0DBF73FEDB892B9EA45B1D6");
    let signature = hex!("6A4E49AA279FE3AF06A5A42B189CE179023FE51EEDEE67131AD855339E6216008270290D89459ED65991C8F4E799C82E558C94ED5A8B95B7FDEA61E5844B2F0C");

    let verifier = Algorithm::Ed25519.get_verifier(&public_key);
    let is_valid = verifier.try_is_valid(MESSAGE, &signature);

    assert!(is_valid.is_ok());

    let is_valid = is_valid.unwrap();
    assert!(is_valid);
}
