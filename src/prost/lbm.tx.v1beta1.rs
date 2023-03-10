/// GetBlockWithTxsRequest is the request type for the Service.GetBlockWithTxs
/// RPC method.
///
/// Since: lbm-sdk 0.47.0
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockWithTxsRequest {
    /// height is the height of the block to query.
    #[prost(int64, tag = "1")]
    pub height: i64,
    /// pagination defines a pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// GetBlockWithTxsResponse is the response type for the Service.GetBlockWithTxs method.
///
/// Since: lbm-sdk 0.47.0
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockWithTxsResponse {
    /// txs are the transactions in the block.
    #[prost(message, repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<super::super::super::cosmos::tx::v1beta1::Tx>,
    #[prost(message, optional, tag = "2")]
    pub block_id: ::core::option::Option<
        super::super::super::tendermint::types::BlockId,
    >,
    #[prost(message, optional, tag = "3")]
    pub block: ::core::option::Option<super::super::super::ostracon::types::Block>,
    /// pagination defines a pagination for the response.
    #[prost(message, optional, tag = "4")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
