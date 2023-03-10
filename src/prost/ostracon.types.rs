/// Entropy represents height-specific complexity and used in proposer-election.
/// Entropy contains vrf proof and generated round. The relationship of each field is as follows.
/// Entropy.proof = VRFProof(last_proof_hash, current_height, Entropy.round)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entropy {
    #[prost(int32, tag = "1")]
    pub round: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::super::tendermint::types::Header>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<super::super::tendermint::types::Data>,
    #[prost(message, optional, tag = "3")]
    pub evidence: ::core::option::Option<super::super::tendermint::types::EvidenceList>,
    #[prost(message, optional, tag = "4")]
    pub last_commit: ::core::option::Option<super::super::tendermint::types::Commit>,
    /// *** Ostracon Extended Fields ***
    #[prost(message, optional, tag = "1000")]
    pub entropy: ::core::option::Option<Entropy>,
}
