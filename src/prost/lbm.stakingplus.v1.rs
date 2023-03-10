/// CreateValidatorAuthorization allows the grantee to create a new validator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateValidatorAuthorization {
    /// redundant, but good for the query.
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
}
