/// InactiveAddr models the blocked address for the bankplus module
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InactiveAddr {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
