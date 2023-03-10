/// Params defines the parameters for the collection module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(uint32, tag = "1")]
    pub depth_limit: u32,
    #[prost(uint32, tag = "2")]
    pub width_limit: u32,
}
/// Contract defines the information of the contract for the collection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contract {
    /// contract_id defines the unique identifier of the contract.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the contract.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// meta is a brief description of the contract.
    #[prost(string, tag = "3")]
    pub meta: ::prost::alloc::string::String,
    /// uri for the contract image stored off chain.
    #[prost(string, tag = "4")]
    pub uri: ::prost::alloc::string::String,
}
/// FTClass defines the class of fungible token.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FtClass {
    /// id defines the unique identifier of the token class.
    /// Note: size of the class id is 8 in length.
    /// Note: token id of the fungible token would be `id` + `00000000`.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the token class.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// meta is a brief description of the token class.
    #[prost(string, tag = "3")]
    pub meta: ::prost::alloc::string::String,
    /// decimals is the number of decimals which one must divide the amount by to get its user representation.
    #[prost(int32, tag = "4")]
    pub decimals: i32,
    /// mintable represents whether the token class is allowed to mint or burn its tokens.
    #[prost(bool, tag = "5")]
    pub mintable: bool,
}
/// NFTClass defines the class of non-fungible token.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NftClass {
    /// id defines the unique identifier of the token class.
    /// Note: size of the class id is 8 in length.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the token class.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// meta is a brief description of the token class.
    #[prost(string, tag = "3")]
    pub meta: ::prost::alloc::string::String,
}
/// NFT defines the information of non-fungible token.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Nft {
    /// token id defines the unique identifier of the token.
    #[prost(string, tag = "1")]
    pub token_id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the token.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// meta is a brief description of the token.
    #[prost(string, tag = "3")]
    pub meta: ::prost::alloc::string::String,
}
/// Deprecated: use NFT
///
/// OwnerNFT defines the information of non-fungible token.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnerNft {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// id defines the unique identifier of the token.
    #[prost(string, tag = "2")]
    pub token_id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the token.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// meta is a brief description of the token.
    #[prost(string, tag = "4")]
    pub meta: ::prost::alloc::string::String,
    /// owner of the token.
    #[prost(string, tag = "5")]
    pub owner: ::prost::alloc::string::String,
}
/// Deprecated: use FTClass
///
/// FT defines the information of fungible token.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ft {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id defines the unique identifier of the fungible token.
    #[prost(string, tag = "2")]
    pub token_id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the fungible token.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// meta is a brief description of the fungible token.
    #[prost(string, tag = "4")]
    pub meta: ::prost::alloc::string::String,
    /// decimals is the number of decimals which one must divide the amount by to get its user representation.
    #[prost(int32, tag = "5")]
    pub decimals: i32,
    /// mintable represents whether the fungible token is allowed to be minted or burnt.
    #[prost(bool, tag = "6")]
    pub mintable: bool,
}
/// Deprecated: use TokenClass
///
/// TokenType defines the information of token type.
/// It represents a NFTClass whose class_id is token_type.
///
/// Note: There is no TokenType instance for FTClass.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenType {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// token type defines the unique identifier of the token type.
    /// the format of the value is identical to that of class_id.
    #[prost(string, tag = "2")]
    pub token_type: ::prost::alloc::string::String,
    /// name defines the human-readable name of the token type.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// meta is a brief description of the token type.
    #[prost(string, tag = "4")]
    pub meta: ::prost::alloc::string::String,
}
/// Coin defines a token with a token id and an amount.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coin {
    /// token id associated with the token.
    #[prost(string, tag = "1")]
    pub token_id: ::prost::alloc::string::String,
    /// amount of the token.
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
/// Grant defines permission given to a grantee.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Grant {
    /// address of the grantee.
    #[prost(string, tag = "1")]
    pub grantee: ::prost::alloc::string::String,
    /// permission on the contract.
    #[prost(enumeration = "Permission", tag = "2")]
    pub permission: i32,
}
/// Authorization defines an authorization given to the operator on tokens of the holder.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authorization {
    /// address of the holder which authorizes the manipulation of its tokens.
    #[prost(string, tag = "1")]
    pub holder: ::prost::alloc::string::String,
    /// address of the operator which the authorization is granted to.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
}
/// Attribute defines a key and value of the attribute.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attribute {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Permission enumerates the valid permissions on a contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Permission {
    /// unspecified defines the default permission which is invalid.
    Unspecified = 0,
    /// PERMISSION_ISSUE defines a permission to create a token class.
    Issue = 1,
    /// PERMISSION_MODIFY defines a permission to modify a contract.
    Modify = 2,
    /// PERMISSION_MINT defines a permission to mint tokens of a contract.
    Mint = 3,
    /// PERMISSION_BURN defines a permission to burn tokens of a contract.
    Burn = 4,
}
impl Permission {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Permission::Unspecified => "PERMISSION_UNSPECIFIED",
            Permission::Issue => "PERMISSION_ISSUE",
            Permission::Modify => "PERMISSION_MODIFY",
            Permission::Mint => "PERMISSION_MINT",
            Permission::Burn => "PERMISSION_BURN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PERMISSION_UNSPECIFIED" => Some(Self::Unspecified),
            "PERMISSION_ISSUE" => Some(Self::Issue),
            "PERMISSION_MODIFY" => Some(Self::Modify),
            "PERMISSION_MINT" => Some(Self::Mint),
            "PERMISSION_BURN" => Some(Self::Burn),
            _ => None,
        }
    }
}
/// Deprecated: use Permission
///
/// LegacyPermission enumerates the valid permissions on a contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LegacyPermission {
    /// unspecified defines the default permission which is invalid.
    Unspecified = 0,
    /// issue defines a permission to create a token class.
    Issue = 1,
    /// modify defines a permission to modify a contract.
    Modify = 2,
    /// mint defines a permission to mint tokens of a contract.
    Mint = 3,
    /// burn defines a permission to burn tokens of a contract.
    Burn = 4,
}
impl LegacyPermission {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LegacyPermission::Unspecified => "LEGACY_PERMISSION_UNSPECIFIED",
            LegacyPermission::Issue => "LEGACY_PERMISSION_ISSUE",
            LegacyPermission::Modify => "LEGACY_PERMISSION_MODIFY",
            LegacyPermission::Mint => "LEGACY_PERMISSION_MINT",
            LegacyPermission::Burn => "LEGACY_PERMISSION_BURN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LEGACY_PERMISSION_UNSPECIFIED" => Some(Self::Unspecified),
            "LEGACY_PERMISSION_ISSUE" => Some(Self::Issue),
            "LEGACY_PERMISSION_MODIFY" => Some(Self::Modify),
            "LEGACY_PERMISSION_MINT" => Some(Self::Mint),
            "LEGACY_PERMISSION_BURN" => Some(Self::Burn),
            _ => None,
        }
    }
}
/// MsgSendFT is the Msg/SendFT request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendFt {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// the address which the transfer is from.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// the address which the transfer is to.
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    /// the amount of the transfer.
    /// Note: amount may be empty.
    #[prost(message, repeated, tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<Coin>,
}
/// MsgSendFTResponse is the Msg/SendFT response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendFtResponse {}
/// MsgOperatorSendFT is the Msg/OperatorSendFT request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgOperatorSendFt {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// the address of the operator.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// the address which the transfer is from.
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// the address which the transfer is to.
    #[prost(string, tag = "4")]
    pub to: ::prost::alloc::string::String,
    /// the amount of the transfer.
    /// Note: amount may be empty.
    #[prost(message, repeated, tag = "5")]
    pub amount: ::prost::alloc::vec::Vec<Coin>,
}
/// MsgOperatorSendFTResponse is the Msg/OperatorSendFT response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgOperatorSendFtResponse {}
/// MsgSendNFT is the Msg/SendNFT request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendNft {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// the address which the transfer is from.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// the address which the transfer is to.
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    /// the token ids to transfer.
    #[prost(string, repeated, tag = "4")]
    pub token_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgSendNFTResponse is the Msg/SendNFT response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendNftResponse {}
/// MsgOperatorSendNFT is the Msg/OperatorSendNFT request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgOperatorSendNft {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// the address of the operator.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// the address which the transfer is from.
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// the address which the transfer is to.
    #[prost(string, tag = "4")]
    pub to: ::prost::alloc::string::String,
    /// the token ids to transfer.
    #[prost(string, repeated, tag = "5")]
    pub token_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgOperatorSendNFTResponse is the Msg/OperatorSendNFT response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgOperatorSendNftResponse {}
/// MsgAuthorizeOperator is the Msg/AuthorizeOperator request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAuthorizeOperator {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the holder who allows the manipulation of its token.
    #[prost(string, tag = "2")]
    pub holder: ::prost::alloc::string::String,
    /// address which the manipulation is allowed to.
    #[prost(string, tag = "3")]
    pub operator: ::prost::alloc::string::String,
}
/// MsgAuthorizeOperatorResponse is the Msg/AuthorizeOperator response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAuthorizeOperatorResponse {}
/// MsgRevokeOperator is the Msg/RevokeOperator request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevokeOperator {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the holder who allows the manipulation of its token.
    #[prost(string, tag = "2")]
    pub holder: ::prost::alloc::string::String,
    /// address which the manipulation is allowed to.
    #[prost(string, tag = "3")]
    pub operator: ::prost::alloc::string::String,
}
/// MsgRevokeOperatorResponse is the Msg/RevokeOperator response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevokeOperatorResponse {}
/// MsgCreateContract is the Msg/CreateContract request type.
///
/// Signer: `owner`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateContract {
    /// address which all the permissions on the contract will be granted to (not a permanent property).
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// name defines the human-readable name of the contract.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// uri for the contract image stored off chain.
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// meta is a brief description of the contract.
    #[prost(string, tag = "4")]
    pub meta: ::prost::alloc::string::String,
}
/// MsgCreateContractResponse is the Msg/CreateContract response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateContractResponse {
    /// id of the new contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
}
/// MsgIssueFT is the Msg/IssueFT request type.
///
/// Signer: `owner`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIssueFt {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the token type.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// meta is a brief description of the token type.
    #[prost(string, tag = "3")]
    pub meta: ::prost::alloc::string::String,
    /// decimals is the number of decimals which one must divide the amount by to get its user representation.
    #[prost(int32, tag = "4")]
    pub decimals: i32,
    /// mintable represents whether the token is allowed to be minted or burnt.
    #[prost(bool, tag = "5")]
    pub mintable: bool,
    /// the address of the grantee which must have the permission to issue a token.
    #[prost(string, tag = "6")]
    pub owner: ::prost::alloc::string::String,
    /// the address to send the minted tokens to. mandatory.
    #[prost(string, tag = "7")]
    pub to: ::prost::alloc::string::String,
    /// the amount of tokens to mint on the issuance.
    /// Note: if you provide negative amount, a panic may result.
    /// Note: amount may be zero.
    #[prost(string, tag = "8")]
    pub amount: ::prost::alloc::string::String,
}
/// MsgIssueFTResponse is the Msg/IssueFT response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIssueFtResponse {
    /// id of the token.
    #[prost(string, tag = "1")]
    pub token_id: ::prost::alloc::string::String,
}
/// MsgIssueNFT is the Msg/IssueNFT request type.
///
/// Signer: `owner`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIssueNft {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the token type.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// meta is a brief description of the token type.
    #[prost(string, tag = "3")]
    pub meta: ::prost::alloc::string::String,
    /// the address of the grantee which must have the permission to issue a token.
    #[prost(string, tag = "4")]
    pub owner: ::prost::alloc::string::String,
}
/// MsgIssueNFTResponse is the Msg/IssueNFT response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIssueNftResponse {
    /// id of the new token type.
    /// refer to TokenType for the definition.
    #[prost(string, tag = "1")]
    pub token_type: ::prost::alloc::string::String,
}
/// MsgMintFT is the Msg/MintFT request type.
///
/// Signer: `from`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMintFt {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the grantee which has the permission for the mint.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// address which the minted tokens will be sent to.
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    /// the amount of the mint.
    /// Note: amount may be empty.
    #[prost(message, repeated, tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<Coin>,
}
/// MsgMintFTResponse is the Msg/MintFT response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMintFtResponse {}
/// MsgMintNFT is the Msg/MintNFT request type.
///
/// Signer: `from`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMintNft {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the grantee which has the permission for the mint.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// address which the minted token will be sent to.
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    /// parameters for the minted tokens.
    #[prost(message, repeated, tag = "4")]
    pub params: ::prost::alloc::vec::Vec<MintNftParam>,
}
/// MsgMintNFTResponse is the Msg/MintNFT response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMintNftResponse {
    /// ids of the new non-fungible tokens.
    #[prost(string, repeated, tag = "1")]
    pub token_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MintNFTParam defines a parameter for minting nft.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintNftParam {
    /// token type or class id of the nft.
    /// Note: it cannot start with zero.
    /// refer to TokenType for the definition.
    #[prost(string, tag = "1")]
    pub token_type: ::prost::alloc::string::String,
    /// name defines the human-readable name of the nft (mandatory).
    /// Note: it has an app-specific limit in length.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// meta is a brief description of the nft.
    /// Note: it has an app-specific limit in length.
    #[prost(string, tag = "3")]
    pub meta: ::prost::alloc::string::String,
}
/// MsgBurnFT is the Msg/BurnFT request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBurnFt {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which the tokens will be burnt from.
    /// Note: it must have the permission for the burn.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// the amount of the burn.
    /// Note: amount may be empty.
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<Coin>,
}
/// MsgBurnFTResponse is the Msg/BurnFT response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBurnFtResponse {}
/// MsgOperatorBurnFT is the Msg/OperatorBurnFT request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgOperatorBurnFt {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggers the burn.
    /// Note: it must have the permission for the burn.
    /// Note: it must have been authorized by from.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// address which the tokens will be burnt from.
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// the amount of the burn.
    /// Note: amount may be empty.
    #[prost(message, repeated, tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<Coin>,
}
/// MsgOperatorBurnFTResponse is the Msg/OperatorBurnFT response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgOperatorBurnFtResponse {}
/// MsgBurnNFT is the Msg/BurnNFT request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBurnNft {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which the tokens will be burnt from.
    /// Note: it must have the permission for the burn.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// the token ids to burn.
    /// Note: id cannot start with zero.
    #[prost(string, repeated, tag = "3")]
    pub token_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgBurnNFTResponse is the Msg/BurnNFT response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBurnNftResponse {}
/// MsgOperatorBurnNFT is the Msg/OperatorBurnNFT request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgOperatorBurnNft {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggers the burn.
    /// Note: it must have the permission for the burn.
    /// Note: it must have been authorized by from.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// address which the tokens will be burnt from.
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// the token ids to burn.
    /// Note: id cannot start with zero.
    #[prost(string, repeated, tag = "4")]
    pub token_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgOperatorBurnNFTResponse is the Msg/OperatorBurnNFT response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgOperatorBurnNftResponse {}
/// MsgModify is the Msg/Modify request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgModify {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// the address of the grantee which must have modify permission.
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// token type of the token.
    /// refer to TokenType for the definition.
    #[prost(string, tag = "3")]
    pub token_type: ::prost::alloc::string::String,
    /// token index of the token.
    /// if index is empty, it would modify the corresponding token type.
    /// if index is not empty, it would modify the corresponding nft.
    /// Note: if token type is of FTs, the index cannot be empty.
    #[prost(string, tag = "4")]
    pub token_index: ::prost::alloc::string::String,
    /// changes to apply.
    /// possible attribute keys on modifying collection: name, uri, base_img_uri (deprecated), meta.
    /// possible attribute keys on modifying token type and token: name, meta.
    #[prost(message, repeated, tag = "5")]
    pub changes: ::prost::alloc::vec::Vec<Attribute>,
}
/// MsgModifyResponse is the Msg/Modify response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgModifyResponse {}
/// MsgGrantPermission is the Msg/GrantPermission request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGrantPermission {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the granter which must have the permission to give.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// address of the grantee.
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    /// permission on the contract.
    #[prost(string, tag = "4")]
    pub permission: ::prost::alloc::string::String,
}
/// MsgGrantPermissionResponse is the Msg/GrantPermission response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGrantPermissionResponse {}
/// MsgRevokePermission is the Msg/RevokePermission request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevokePermission {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the grantee which abandons the permission.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// permission on the contract.
    #[prost(string, tag = "3")]
    pub permission: ::prost::alloc::string::String,
}
/// MsgRevokePermissionResponse is the Msg/RevokePermission response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevokePermissionResponse {}
/// MsgAttach is the Msg/Attach request type.
///
/// Signer: `from`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAttach {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the owner of the token.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// token id of the token to attach.
    #[prost(string, tag = "3")]
    pub token_id: ::prost::alloc::string::String,
    /// to token id which one attachs the token to.
    #[prost(string, tag = "4")]
    pub to_token_id: ::prost::alloc::string::String,
}
/// MsgAttachResponse is the Msg/Attach response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAttachResponse {}
/// MsgDetach is the Msg/Detach request type.
///
/// Signer: `from`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDetach {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the owner of the token.
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// token id of the token to detach.
    #[prost(string, tag = "3")]
    pub token_id: ::prost::alloc::string::String,
}
/// MsgDetachResponse is the Msg/Detach response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDetachResponse {}
/// MsgOperatorAttach is the Msg/OperatorAttach request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgOperatorAttach {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the operator.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// address of the owner of the token.
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// token id of the token to attach.
    #[prost(string, tag = "4")]
    pub token_id: ::prost::alloc::string::String,
    /// to token id which one attachs the token to.
    #[prost(string, tag = "5")]
    pub to_token_id: ::prost::alloc::string::String,
}
/// MsgOperatorAttachResponse is the Msg/OperatorAttach response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgOperatorAttachResponse {}
/// MsgOperatorDetach is the Msg/OperatorDetach request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgOperatorDetach {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the operator.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// address of the owner of the token.
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// token id of the token to detach.
    #[prost(string, tag = "4")]
    pub token_id: ::prost::alloc::string::String,
}
/// MsgOperatorDetachResponse is the Msg/OperatorDetach response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgOperatorDetachResponse {}
/// GenesisState defines the collection module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// contracts defines the metadata of the contracts.
    #[prost(message, repeated, tag = "2")]
    pub contracts: ::prost::alloc::vec::Vec<Contract>,
    /// next ids for token classes.
    #[prost(message, repeated, tag = "3")]
    pub next_class_ids: ::prost::alloc::vec::Vec<NextClassIDs>,
    /// classes defines the metadata of the tokens.
    #[prost(message, repeated, tag = "4")]
    pub classes: ::prost::alloc::vec::Vec<ContractClasses>,
    /// next ids for (non-fungible) tokens.
    #[prost(message, repeated, tag = "5")]
    pub next_token_ids: ::prost::alloc::vec::Vec<ContractNextTokenIDs>,
    /// balances is an array containing the balances of all the accounts.
    #[prost(message, repeated, tag = "6")]
    pub balances: ::prost::alloc::vec::Vec<ContractBalances>,
    /// nfts is an array containing the nfts.
    #[prost(message, repeated, tag = "7")]
    pub nfts: ::prost::alloc::vec::Vec<ContractNfTs>,
    /// parents represents the parents of (non-fungible) tokens.
    #[prost(message, repeated, tag = "8")]
    pub parents: ::prost::alloc::vec::Vec<ContractTokenRelations>,
    /// grants defines the grant information.
    #[prost(message, repeated, tag = "9")]
    pub grants: ::prost::alloc::vec::Vec<ContractGrants>,
    /// authorizations defines the approve information.
    #[prost(message, repeated, tag = "10")]
    pub authorizations: ::prost::alloc::vec::Vec<ContractAuthorizations>,
    /// supplies represents the total supplies of tokens.
    #[prost(message, repeated, tag = "11")]
    pub supplies: ::prost::alloc::vec::Vec<ContractStatistics>,
    /// burnts represents the total amount of burnt tokens.
    #[prost(message, repeated, tag = "12")]
    pub burnts: ::prost::alloc::vec::Vec<ContractStatistics>,
}
/// ContractBalances defines balances belong to a contract.
/// genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractBalances {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// balances
    #[prost(message, repeated, tag = "2")]
    pub balances: ::prost::alloc::vec::Vec<Balance>,
}
/// ContractStatistics defines statistics belong to a contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractStatistics {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// statistics
    #[prost(message, repeated, tag = "2")]
    pub statistics: ::prost::alloc::vec::Vec<ClassStatistics>,
}
/// ClassStatistics defines statistics belong to a token class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassStatistics {
    /// class id associated with the token class.
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// statistics
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
/// Balance defines a balance of an address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Balance {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub amount: ::prost::alloc::vec::Vec<Coin>,
}
/// ContractClasses defines token classes belong to a contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractClasses {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// classes
    #[prost(message, repeated, tag = "2")]
    pub classes: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// ContractNFTs defines token classes belong to a contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractNfTs {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// nfts
    #[prost(message, repeated, tag = "2")]
    pub nfts: ::prost::alloc::vec::Vec<Nft>,
}
/// ContractAuthorizations defines authorizations belong to a contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractAuthorizations {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// authorizations
    #[prost(message, repeated, tag = "2")]
    pub authorizations: ::prost::alloc::vec::Vec<Authorization>,
}
/// ContractGrant defines grants belong to a contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractGrants {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// grants
    #[prost(message, repeated, tag = "2")]
    pub grants: ::prost::alloc::vec::Vec<Grant>,
}
/// NextClassIDs defines the next class ids of the contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NextClassIDs {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// id for the fungible tokens.
    #[prost(string, tag = "2")]
    pub fungible: ::prost::alloc::string::String,
    /// id for the non-fungible tokens.
    #[prost(string, tag = "3")]
    pub non_fungible: ::prost::alloc::string::String,
}
/// ContractNextTokenIDs defines the next token ids belong to a contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractNextTokenIDs {
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub token_ids: ::prost::alloc::vec::Vec<NextTokenId>,
}
/// NextTokenID defines the next (non-fungible) token id of the token class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NextTokenId {
    /// class id associated with the token class.
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// id for the token.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
/// ContractTokenRelations defines token relations belong to a contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractTokenRelations {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// relations
    #[prost(message, repeated, tag = "2")]
    pub relations: ::prost::alloc::vec::Vec<TokenRelation>,
}
/// TokenRelation defines relations between two tokens.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenRelation {
    /// self
    #[prost(string, tag = "1")]
    pub self_: ::prost::alloc::string::String,
    /// other
    #[prost(string, tag = "2")]
    pub other: ::prost::alloc::string::String,
}
/// EventSent is emitted when tokens are transferred.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSent {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the send.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// holder whose tokens were sent.
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// recipient of the tokens.
    #[prost(string, tag = "4")]
    pub to: ::prost::alloc::string::String,
    /// amount of tokens sent.
    #[prost(message, repeated, tag = "5")]
    pub amount: ::prost::alloc::vec::Vec<Coin>,
}
/// EventAuthorizedOperator is emitted when a holder authorizes an operator to manipulate its tokens.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAuthorizedOperator {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of a holder which authorized the `operator` address as an operator.
    #[prost(string, tag = "2")]
    pub holder: ::prost::alloc::string::String,
    /// address which became an operator of `holder`.
    #[prost(string, tag = "3")]
    pub operator: ::prost::alloc::string::String,
}
/// EventRevokedOperator is emitted when an authorization is revoked.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRevokedOperator {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of a holder which revoked the `operator` address as an operator.
    #[prost(string, tag = "2")]
    pub holder: ::prost::alloc::string::String,
    /// address which was revoked as an operator of `holder`.
    #[prost(string, tag = "3")]
    pub operator: ::prost::alloc::string::String,
}
/// EventCreatedContract is emitted when a new contract is created.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreatedContract {
    /// address which created the contract.
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// contract id associated with the contract.
    #[prost(string, tag = "2")]
    pub contract_id: ::prost::alloc::string::String,
    /// name of the contract.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// metadata of the contract.
    #[prost(string, tag = "4")]
    pub meta: ::prost::alloc::string::String,
    /// uri for the contract image stored off chain.
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
}
/// EventCreatedFTClass is emitted when a new fungible token class is created.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreatedFtClass {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the create.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// token id associated with the token class.
    #[prost(string, tag = "3")]
    pub token_id: ::prost::alloc::string::String,
    /// name of the token class.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// metadata of the token class.
    #[prost(string, tag = "5")]
    pub meta: ::prost::alloc::string::String,
    /// decimals of the token class.
    #[prost(int32, tag = "6")]
    pub decimals: i32,
    /// mintable represents whether the token class is allowed to mint or burn its tokens.
    #[prost(bool, tag = "7")]
    pub mintable: bool,
}
/// EventCreatedNFTClass is emitted when a new non-fungible token class is created.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreatedNftClass {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the create.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// token type associated with the token class.
    /// refer to TokenType for the definition.
    #[prost(string, tag = "3")]
    pub token_type: ::prost::alloc::string::String,
    /// name of the token class.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// metadata of the token class.
    #[prost(string, tag = "5")]
    pub meta: ::prost::alloc::string::String,
}
/// EventGranted is emitted when a granter grants its permission to a grantee.
///
/// Info: `granter` would be empty if the permission is granted by an issuance.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventGranted {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the granter which grants the permission.
    #[prost(string, tag = "2")]
    pub granter: ::prost::alloc::string::String,
    /// address of the grantee.
    #[prost(string, tag = "3")]
    pub grantee: ::prost::alloc::string::String,
    /// permission on the contract.
    #[prost(enumeration = "Permission", tag = "4")]
    pub permission: i32,
}
/// EventRenounced is emitted when a grantee renounced its permission.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRenounced {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the grantee which abandons its grant.
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    /// permission on the contract.
    #[prost(enumeration = "Permission", tag = "3")]
    pub permission: i32,
}
/// EventMintedFT is emitted when fungible tokens are minted.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMintedFt {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the mint.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// recipient of the tokens.
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    /// amount of tokens minted.
    #[prost(message, repeated, tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<Coin>,
}
/// EventMintedNFT is emitted when non-fungible tokens are minted.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMintedNft {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the mint.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// recipient of the tokens.
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    /// tokens minted.
    #[prost(message, repeated, tag = "4")]
    pub tokens: ::prost::alloc::vec::Vec<Nft>,
}
/// EventBurned is emitted when tokens are burnt.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBurned {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the burn.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// holder whose tokens were burned.
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// amount of tokens burned.
    #[prost(message, repeated, tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<Coin>,
}
/// EventModifiedContract is emitted when the information of a contract is modified.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventModifiedContract {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the modify.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// changes of the attributes applied.
    /// possible attribute keys are same as those of MsgModify.
    #[prost(message, repeated, tag = "3")]
    pub changes: ::prost::alloc::vec::Vec<Attribute>,
}
/// EventModifiedTokenClass is emitted when the information of a token class is modified.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventModifiedTokenClass {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the modify.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// token type associated with the token class.
    /// refer to TokenType for the definition.
    #[prost(string, tag = "3")]
    pub token_type: ::prost::alloc::string::String,
    /// changes of the attributes applied.
    /// possible attribute keys are same as those of MsgModify.
    #[prost(message, repeated, tag = "4")]
    pub changes: ::prost::alloc::vec::Vec<Attribute>,
    /// type name of the token class.
    #[prost(string, tag = "5")]
    pub type_name: ::prost::alloc::string::String,
}
/// EventModifiedNFT is emitted when the information of a non-fungible token is modified.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventModifiedNft {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the modify.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// token id associated with the non-fungible token.
    #[prost(string, tag = "3")]
    pub token_id: ::prost::alloc::string::String,
    /// changes of the attributes applied.
    /// possible attribute keys are same as those of MsgModify.
    #[prost(message, repeated, tag = "4")]
    pub changes: ::prost::alloc::vec::Vec<Attribute>,
}
/// EventAttached is emitted when a token is attached to another.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAttached {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the attach.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// address which holds the tokens.
    #[prost(string, tag = "3")]
    pub holder: ::prost::alloc::string::String,
    /// subject of the attach.
    #[prost(string, tag = "4")]
    pub subject: ::prost::alloc::string::String,
    /// target of the attach.
    #[prost(string, tag = "5")]
    pub target: ::prost::alloc::string::String,
}
/// EventDetached is emitted when a token is detached from its parent.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDetached {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address which triggered the detach.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// address which holds the token.
    #[prost(string, tag = "3")]
    pub holder: ::prost::alloc::string::String,
    /// token being detached.
    #[prost(string, tag = "4")]
    pub subject: ::prost::alloc::string::String,
    /// parent token before the detach.
    #[prost(string, tag = "5")]
    pub previous_parent: ::prost::alloc::string::String,
}
/// EventOwnerChanged is emitted when the owner of token is changed by operation applied to its ancestor.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventOwnerChanged {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id associated with the token.
    #[prost(string, tag = "2")]
    pub token_id: ::prost::alloc::string::String,
    /// address of the previous owner before the change.
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// address of the new owner.
    #[prost(string, tag = "4")]
    pub to: ::prost::alloc::string::String,
}
/// EventRootChanged is emitted when the root of token is changed by operation applied to its ancestor.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRootChanged {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id associated with the token.
    #[prost(string, tag = "2")]
    pub token_id: ::prost::alloc::string::String,
    /// token id of the previous root before the change.
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// token id of the new root.
    #[prost(string, tag = "4")]
    pub to: ::prost::alloc::string::String,
}
/// Deprecated: use typed events.
///
/// EventType enumerates the valid event types on x/collection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventType {
    Unspecified = 0,
    CreateCollection = 1,
    IssueFt = 2,
    IssueNft = 3,
    MintFt = 4,
    BurnFt = 5,
    MintNft = 6,
    BurnNft = 7,
    BurnFtFrom = 8,
    BurnNftFrom = 9,
    ModifyCollection = 10,
    ModifyTokenType = 11,
    ModifyToken = 12,
    Transfer = 13,
    TransferFt = 14,
    TransferNft = 15,
    TransferFtFrom = 16,
    TransferNftFrom = 17,
    GrantPerm = 18,
    RevokePerm = 19,
    Attach = 20,
    Detach = 21,
    AttachFrom = 22,
    DetachFrom = 23,
    ApproveCollection = 24,
    DisapproveCollection = 25,
    OperationTransferNft = 26,
    OperationBurnNft = 27,
    OperationRootChanged = 28,
}
impl EventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventType::Unspecified => "EVENT_TYPE_UNSPECIFIED",
            EventType::CreateCollection => "EVENT_TYPE_CREATE_COLLECTION",
            EventType::IssueFt => "EVENT_TYPE_ISSUE_FT",
            EventType::IssueNft => "EVENT_TYPE_ISSUE_NFT",
            EventType::MintFt => "EVENT_TYPE_MINT_FT",
            EventType::BurnFt => "EVENT_TYPE_BURN_FT",
            EventType::MintNft => "EVENT_TYPE_MINT_NFT",
            EventType::BurnNft => "EVENT_TYPE_BURN_NFT",
            EventType::BurnFtFrom => "EVENT_TYPE_BURN_FT_FROM",
            EventType::BurnNftFrom => "EVENT_TYPE_BURN_NFT_FROM",
            EventType::ModifyCollection => "EVENT_TYPE_MODIFY_COLLECTION",
            EventType::ModifyTokenType => "EVENT_TYPE_MODIFY_TOKEN_TYPE",
            EventType::ModifyToken => "EVENT_TYPE_MODIFY_TOKEN",
            EventType::Transfer => "EVENT_TYPE_TRANSFER",
            EventType::TransferFt => "EVENT_TYPE_TRANSFER_FT",
            EventType::TransferNft => "EVENT_TYPE_TRANSFER_NFT",
            EventType::TransferFtFrom => "EVENT_TYPE_TRANSFER_FT_FROM",
            EventType::TransferNftFrom => "EVENT_TYPE_TRANSFER_NFT_FROM",
            EventType::GrantPerm => "EVENT_TYPE_GRANT_PERM",
            EventType::RevokePerm => "EVENT_TYPE_REVOKE_PERM",
            EventType::Attach => "EVENT_TYPE_ATTACH",
            EventType::Detach => "EVENT_TYPE_DETACH",
            EventType::AttachFrom => "EVENT_TYPE_ATTACH_FROM",
            EventType::DetachFrom => "EVENT_TYPE_DETACH_FROM",
            EventType::ApproveCollection => "EVENT_TYPE_APPROVE_COLLECTION",
            EventType::DisapproveCollection => "EVENT_TYPE_DISAPPROVE_COLLECTION",
            EventType::OperationTransferNft => "EVENT_TYPE_OPERATION_TRANSFER_NFT",
            EventType::OperationBurnNft => "EVENT_TYPE_OPERATION_BURN_NFT",
            EventType::OperationRootChanged => "EVENT_TYPE_OPERATION_ROOT_CHANGED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EVENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "EVENT_TYPE_CREATE_COLLECTION" => Some(Self::CreateCollection),
            "EVENT_TYPE_ISSUE_FT" => Some(Self::IssueFt),
            "EVENT_TYPE_ISSUE_NFT" => Some(Self::IssueNft),
            "EVENT_TYPE_MINT_FT" => Some(Self::MintFt),
            "EVENT_TYPE_BURN_FT" => Some(Self::BurnFt),
            "EVENT_TYPE_MINT_NFT" => Some(Self::MintNft),
            "EVENT_TYPE_BURN_NFT" => Some(Self::BurnNft),
            "EVENT_TYPE_BURN_FT_FROM" => Some(Self::BurnFtFrom),
            "EVENT_TYPE_BURN_NFT_FROM" => Some(Self::BurnNftFrom),
            "EVENT_TYPE_MODIFY_COLLECTION" => Some(Self::ModifyCollection),
            "EVENT_TYPE_MODIFY_TOKEN_TYPE" => Some(Self::ModifyTokenType),
            "EVENT_TYPE_MODIFY_TOKEN" => Some(Self::ModifyToken),
            "EVENT_TYPE_TRANSFER" => Some(Self::Transfer),
            "EVENT_TYPE_TRANSFER_FT" => Some(Self::TransferFt),
            "EVENT_TYPE_TRANSFER_NFT" => Some(Self::TransferNft),
            "EVENT_TYPE_TRANSFER_FT_FROM" => Some(Self::TransferFtFrom),
            "EVENT_TYPE_TRANSFER_NFT_FROM" => Some(Self::TransferNftFrom),
            "EVENT_TYPE_GRANT_PERM" => Some(Self::GrantPerm),
            "EVENT_TYPE_REVOKE_PERM" => Some(Self::RevokePerm),
            "EVENT_TYPE_ATTACH" => Some(Self::Attach),
            "EVENT_TYPE_DETACH" => Some(Self::Detach),
            "EVENT_TYPE_ATTACH_FROM" => Some(Self::AttachFrom),
            "EVENT_TYPE_DETACH_FROM" => Some(Self::DetachFrom),
            "EVENT_TYPE_APPROVE_COLLECTION" => Some(Self::ApproveCollection),
            "EVENT_TYPE_DISAPPROVE_COLLECTION" => Some(Self::DisapproveCollection),
            "EVENT_TYPE_OPERATION_TRANSFER_NFT" => Some(Self::OperationTransferNft),
            "EVENT_TYPE_OPERATION_BURN_NFT" => Some(Self::OperationBurnNft),
            "EVENT_TYPE_OPERATION_ROOT_CHANGED" => Some(Self::OperationRootChanged),
            _ => None,
        }
    }
}
/// Deprecated: use typed events.
///
/// AttributeKey enumerates the valid attribute keys on x/collection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AttributeKey {
    Unspecified = 0,
    Name = 1,
    Meta = 2,
    ContractId = 3,
    TokenId = 4,
    Owner = 5,
    Amount = 6,
    Decimals = 7,
    /// deprecated: use ATTRIBUTE_KEY_URI
    BaseImgUri = 8,
    Mintable = 9,
    TokenType = 10,
    From = 11,
    To = 12,
    Perm = 13,
    ToTokenId = 14,
    FromTokenId = 15,
    Approver = 16,
    Proxy = 17,
    OldRootTokenId = 18,
    NewRootTokenId = 19,
    Uri = 20,
}
impl AttributeKey {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AttributeKey::Unspecified => "ATTRIBUTE_KEY_UNSPECIFIED",
            AttributeKey::Name => "ATTRIBUTE_KEY_NAME",
            AttributeKey::Meta => "ATTRIBUTE_KEY_META",
            AttributeKey::ContractId => "ATTRIBUTE_KEY_CONTRACT_ID",
            AttributeKey::TokenId => "ATTRIBUTE_KEY_TOKEN_ID",
            AttributeKey::Owner => "ATTRIBUTE_KEY_OWNER",
            AttributeKey::Amount => "ATTRIBUTE_KEY_AMOUNT",
            AttributeKey::Decimals => "ATTRIBUTE_KEY_DECIMALS",
            AttributeKey::BaseImgUri => "ATTRIBUTE_KEY_BASE_IMG_URI",
            AttributeKey::Mintable => "ATTRIBUTE_KEY_MINTABLE",
            AttributeKey::TokenType => "ATTRIBUTE_KEY_TOKEN_TYPE",
            AttributeKey::From => "ATTRIBUTE_KEY_FROM",
            AttributeKey::To => "ATTRIBUTE_KEY_TO",
            AttributeKey::Perm => "ATTRIBUTE_KEY_PERM",
            AttributeKey::ToTokenId => "ATTRIBUTE_KEY_TO_TOKEN_ID",
            AttributeKey::FromTokenId => "ATTRIBUTE_KEY_FROM_TOKEN_ID",
            AttributeKey::Approver => "ATTRIBUTE_KEY_APPROVER",
            AttributeKey::Proxy => "ATTRIBUTE_KEY_PROXY",
            AttributeKey::OldRootTokenId => "ATTRIBUTE_KEY_OLD_ROOT_TOKEN_ID",
            AttributeKey::NewRootTokenId => "ATTRIBUTE_KEY_NEW_ROOT_TOKEN_ID",
            AttributeKey::Uri => "ATTRIBUTE_KEY_URI",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ATTRIBUTE_KEY_UNSPECIFIED" => Some(Self::Unspecified),
            "ATTRIBUTE_KEY_NAME" => Some(Self::Name),
            "ATTRIBUTE_KEY_META" => Some(Self::Meta),
            "ATTRIBUTE_KEY_CONTRACT_ID" => Some(Self::ContractId),
            "ATTRIBUTE_KEY_TOKEN_ID" => Some(Self::TokenId),
            "ATTRIBUTE_KEY_OWNER" => Some(Self::Owner),
            "ATTRIBUTE_KEY_AMOUNT" => Some(Self::Amount),
            "ATTRIBUTE_KEY_DECIMALS" => Some(Self::Decimals),
            "ATTRIBUTE_KEY_BASE_IMG_URI" => Some(Self::BaseImgUri),
            "ATTRIBUTE_KEY_MINTABLE" => Some(Self::Mintable),
            "ATTRIBUTE_KEY_TOKEN_TYPE" => Some(Self::TokenType),
            "ATTRIBUTE_KEY_FROM" => Some(Self::From),
            "ATTRIBUTE_KEY_TO" => Some(Self::To),
            "ATTRIBUTE_KEY_PERM" => Some(Self::Perm),
            "ATTRIBUTE_KEY_TO_TOKEN_ID" => Some(Self::ToTokenId),
            "ATTRIBUTE_KEY_FROM_TOKEN_ID" => Some(Self::FromTokenId),
            "ATTRIBUTE_KEY_APPROVER" => Some(Self::Approver),
            "ATTRIBUTE_KEY_PROXY" => Some(Self::Proxy),
            "ATTRIBUTE_KEY_OLD_ROOT_TOKEN_ID" => Some(Self::OldRootTokenId),
            "ATTRIBUTE_KEY_NEW_ROOT_TOKEN_ID" => Some(Self::NewRootTokenId),
            "ATTRIBUTE_KEY_URI" => Some(Self::Uri),
            _ => None,
        }
    }
}
/// QueryBalanceRequest is the request type for the Query/Balance RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address is the address to query the balance for.
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    /// token id associated with the token.
    #[prost(string, tag = "3")]
    pub token_id: ::prost::alloc::string::String,
}
/// QueryBalanceResponse is the response type for the Query/Balance RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceResponse {
    /// balance is the balance of the token.
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<Coin>,
}
/// QueryAllBalancesRequest is the request type for the Query/AllBalances RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllBalancesRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address is the address to query the balances for.
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryAllBalancesResponse is the response type for the Query/AllBalances RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllBalancesResponse {
    /// balances is the balalces of all the tokens.
    #[prost(message, repeated, tag = "1")]
    pub balances: ::prost::alloc::vec::Vec<Coin>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryFTSupplyRequest is the request type for the Query/FTSupply RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFtSupplyRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id associated with the fungible token.
    #[prost(string, tag = "2")]
    pub token_id: ::prost::alloc::string::String,
}
/// QueryFTSupplyResponse is the response type for the Query/FTSupply RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFtSupplyResponse {
    /// supply is the supply of the tokens.
    #[prost(string, tag = "1")]
    pub supply: ::prost::alloc::string::String,
}
/// QueryFTMintedRequest is the request type for the Query/FTMinted RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFtMintedRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id associated with the fungible token.
    #[prost(string, tag = "2")]
    pub token_id: ::prost::alloc::string::String,
}
/// QueryFTMintedResponse is the response type for the Query/FTMinted RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFtMintedResponse {
    /// minted is the amount of the minted tokens.
    #[prost(string, tag = "1")]
    pub minted: ::prost::alloc::string::String,
}
/// QueryFTBurntRequest is the request type for the Query/FTBurnt RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFtBurntRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id associated with the fungible token.
    #[prost(string, tag = "2")]
    pub token_id: ::prost::alloc::string::String,
}
/// QueryFTBurntResponse is the response type for the Query/FTBurnt RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFtBurntResponse {
    /// burnt is the amount of the burnt tokens.
    #[prost(string, tag = "1")]
    pub burnt: ::prost::alloc::string::String,
}
/// QueryNFTSupplyRequest is the request type for the Query/NFTSupply RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNftSupplyRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// token type associated with the token type.
    /// refer to TokenType for the definition.
    #[prost(string, tag = "2")]
    pub token_type: ::prost::alloc::string::String,
}
/// QueryNFTSupplyResponse is the response type for the Query/NFTSupply RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNftSupplyResponse {
    /// supply is the supply of the non-fungible token.
    #[prost(string, tag = "1")]
    pub supply: ::prost::alloc::string::String,
}
/// QueryNFTMintedRequest is the request type for the Query/NFTMinted RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNftMintedRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// token type associated with the token type.
    /// refer to TokenType for the definition.
    #[prost(string, tag = "2")]
    pub token_type: ::prost::alloc::string::String,
}
/// QueryNFTMintedResponse is the response type for the Query/NFTMinted RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNftMintedResponse {
    /// minted is the amount of minted tokens.
    #[prost(string, tag = "1")]
    pub minted: ::prost::alloc::string::String,
}
/// QueryNFTBurntRequest is the request type for the Query/NFTBurnt RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNftBurntRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// token type associated with the token type.
    /// refer to TokenType for the definition.
    #[prost(string, tag = "2")]
    pub token_type: ::prost::alloc::string::String,
}
/// QueryNFTBurntResponse is the response type for the Query/NFTBurnt RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNftBurntResponse {
    /// burnt is the amount of the burnt tokens.
    #[prost(string, tag = "1")]
    pub burnt: ::prost::alloc::string::String,
}
/// QueryContractRequest is the request type for the Query/Contract RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
}
/// QueryContractResponse is the response type for the Query/Contract RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractResponse {
    /// contract is the information of the contract.
    #[prost(message, optional, tag = "1")]
    pub contract: ::core::option::Option<Contract>,
}
/// QueryTokenClassTypeNameRequest is the request type for the Query/TokenClassTypeName RPC method.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenClassTypeNameRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// class id associated with the token class.
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
}
/// QueryTokenClassTypeNameResponse is the response type for the Query/TokenClassTypeName RPC method.
///
/// Since: 0.46.0 (finschia)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenClassTypeNameResponse {
    /// type name of the token class.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// QueryTokenTypeRequest is the request type for the Query/TokenType RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenTypeRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// token type associated with the token type.
    /// refer to TokenType for the definition.
    #[prost(string, tag = "2")]
    pub token_type: ::prost::alloc::string::String,
}
/// QueryTokenTypeResponse is the response type for the Query/TokenType RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenTypeResponse {
    /// token type is the information of the token type.
    #[prost(message, optional, tag = "1")]
    pub token_type: ::core::option::Option<TokenType>,
}
/// QueryTokenRequest is the request type for the Query/Token RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id associated with the fungible token.
    #[prost(string, tag = "2")]
    pub token_id: ::prost::alloc::string::String,
}
/// QueryTokenResponse is the response type for the Query/Token RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenResponse {
    /// information of the token.
    #[prost(message, optional, tag = "1")]
    pub token: ::core::option::Option<::prost_types::Any>,
}
/// QueryRootRequest is the request type for the Query/Root RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRootRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id associated with the non-fungible token.
    #[prost(string, tag = "2")]
    pub token_id: ::prost::alloc::string::String,
}
/// QueryRootResponse is the response type for the Query/Root RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRootResponse {
    /// root is the information of the root token.
    /// it would return itself if it's the root token.
    #[prost(message, optional, tag = "1")]
    pub root: ::core::option::Option<Nft>,
}
/// QueryParentRequest is the request type for the Query/Parent RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParentRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id associated wit the non-fungible token.
    #[prost(string, tag = "2")]
    pub token_id: ::prost::alloc::string::String,
}
/// QueryParentResponse is the response type for the Query/Parent RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParentResponse {
    /// parent is the information of the parent token.
    /// if there is no parent for the token, it would return nil.
    #[prost(message, optional, tag = "1")]
    pub parent: ::core::option::Option<Nft>,
}
/// QueryChildrenRequest is the request type for the Query/Children RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChildrenRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// token id associated with the non-fungible token.
    #[prost(string, tag = "2")]
    pub token_id: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryChildrenResponse is the response type for the Query/Children RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChildrenResponse {
    /// children is the information of the child tokens.
    #[prost(message, repeated, tag = "1")]
    pub children: ::prost::alloc::vec::Vec<Nft>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryGranteeGrantsRequest is the request type for the Query/GranteeGrants RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGranteeGrantsRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// the address of the grantee.
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryGranteeGrantsResponse is the response type for the Query/GranteeGrants RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGranteeGrantsResponse {
    #[prost(message, repeated, tag = "1")]
    pub grants: ::prost::alloc::vec::Vec<Grant>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryIsOperatorForRequest is the request type for the Query/IsOperatorFor RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIsOperatorForRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// the address of the operator.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// the address of the token holder.
    #[prost(string, tag = "3")]
    pub holder: ::prost::alloc::string::String,
}
/// QueryIsOperatorForResponse is the response type for the Query/IsOperatorFor RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIsOperatorForResponse {
    #[prost(bool, tag = "1")]
    pub authorized: bool,
}
/// QueryHoldersByOperatorRequest is the request type for the Query/HoldersByOperator RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHoldersByOperatorRequest {
    /// contract id associated with the contract.
    #[prost(string, tag = "1")]
    pub contract_id: ::prost::alloc::string::String,
    /// address of the operator.
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryHoldersByOperatorResponse is the response type for the Query/HoldersByOperator RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHoldersByOperatorResponse {
    #[prost(string, repeated, tag = "1")]
    pub holders: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
