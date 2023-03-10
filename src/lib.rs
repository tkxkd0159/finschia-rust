include!("prost/_include.rs");

#[test]
fn test_load_proto() {
    let is_send = cosmos::bank::v1beta1::SendEnabled { denom: String::from("uatom"), enabled: true }.enabled;
    assert!(is_send)
}