fn main() {
    println!("{:?}", finschia_rust::cosmos::bank::v1beta1::SendEnabled { denom: String::from("uatom"), enabled: true });
}