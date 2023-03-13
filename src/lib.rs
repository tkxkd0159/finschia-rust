include!("prost/_include.rs");
mod msg;

pub use msg::{from_any, to_any};

#[test]
fn test_proto_scheme() {
    let is_send = cosmos::bank::v1beta1::SendEnabled {
        denom: String::from("cony"),
        enabled: true,
    }
    .enabled;
    assert!(is_send);

    let msg = cosmos::bank::v1beta1::MsgSend {
        from_address: "me".to_string(),
        to_address: "you".to_string(),
        amount: vec![cosmos::base::v1beta1::Coin {
            denom: "cony".to_string(),
            amount: "100".to_string(),
        }],
    };

    let anymsg = to_any(&msg, "/cosmos.bank.v1beta1.MsgSend");
    assert_eq!(anymsg.type_url, "/cosmos.bank.v1beta1.MsgSend");
    let origmsg = from_any::<cosmos::bank::v1beta1::MsgSend>(&anymsg).unwrap();
    assert_eq!(msg, origmsg);
}
