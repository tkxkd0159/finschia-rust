pub mod ostracon {
    pub mod types {
        include!("prost/ostracon.types.rs");
    }
    pub mod abci {
        include!("prost/ostracon.abci.rs");
    }
}
pub mod tendermint {
    pub mod abci {
        include!("prost/tendermint.abci.rs");
    }
    pub mod crypto {
        include!("prost/tendermint.crypto.rs");
    }
    pub mod p2p {
        include!("prost/tendermint.p2p.rs");
    }
    pub mod types {
        include!("prost/tendermint.types.rs");
    }
    pub mod version {
        include!("prost/tendermint.version.rs");
    }
}
pub mod cosmos {
    pub mod base {
        pub mod abci {
            pub mod v1beta1 {
                include!("prost/cosmos.base.abci.v1beta1.rs");
            }
        }
    }
    pub mod upgrade {
        pub mod v1beta1 {
            include!("prost/cosmos.upgrade.v1beta1.rs");
        }
    }
    pub mod base {
        pub mod node {
            pub mod v1beta1 {
                include!("prost/cosmos.base.node.v1beta1.rs");
            }
        }
    }
    pub mod capability {
        pub mod v1beta1 {
            include!("prost/cosmos.capability.v1beta1.rs");
        }
    }
    pub mod crypto {
        pub mod secp256k1 {
            include!("prost/cosmos.crypto.secp256k1.rs");
        }
    }
    pub mod crisis {
        pub mod v1beta1 {
            include!("prost/cosmos.crisis.v1beta1.rs");
        }
    }
    pub mod mint {
        pub mod v1beta1 {
            include!("prost/cosmos.mint.v1beta1.rs");
        }
    }
    pub mod genutil {
        pub mod v1beta1 {
            include!("prost/cosmos.genutil.v1beta1.rs");
        }
    }
    pub mod params {
        pub mod v1beta1 {
            include!("prost/cosmos.params.v1beta1.rs");
        }
    }
    pub mod staking {
        pub mod v1beta1 {
            include!("prost/cosmos.staking.v1beta1.rs");
        }
    }
    pub mod distribution {
        pub mod v1beta1 {
            include!("prost/cosmos.distribution.v1beta1.rs");
        }
    }
    pub mod base {
        pub mod store {
            pub mod v1beta1 {
                include!("prost/cosmos.base.store.v1beta1.rs");
            }
        }
    }
    pub mod base {
        pub mod kv {
            pub mod v1beta1 {
                include!("prost/cosmos.base.kv.v1beta1.rs");
            }
        }
    }
    pub mod crypto {
        pub mod secp256r1 {
            include!("prost/cosmos.crypto.secp256r1.rs");
        }
    }
    pub mod base {
        pub mod query {
            pub mod v1beta1 {
                include!("prost/cosmos.base.query.v1beta1.rs");
            }
        }
    }
    pub mod base {
        pub mod v1beta1 {
            include!("prost/cosmos.base.v1beta1.rs");
        }
    }
    pub mod tx {
        pub mod v1beta1 {
            include!("prost/cosmos.tx.v1beta1.rs");
        }
    }
    pub mod gov {
        pub mod v1beta1 {
            include!("prost/cosmos.gov.v1beta1.rs");
        }
    }
    pub mod feegrant {
        pub mod v1beta1 {
            include!("prost/cosmos.feegrant.v1beta1.rs");
        }
    }
    pub mod bank {
        pub mod v1beta1 {
            include!("prost/cosmos.bank.v1beta1.rs");
        }
    }
    pub mod base {
        pub mod snapshots {
            pub mod v1beta1 {
                include!("prost/cosmos.base.snapshots.v1beta1.rs");
            }
        }
    }
    pub mod base {
        pub mod reflection {
            pub mod v1beta1 {
                include!("prost/cosmos.base.reflection.v1beta1.rs");
            }
        }
    }
    pub mod slashing {
        pub mod v1beta1 {
            include!("prost/cosmos.slashing.v1beta1.rs");
        }
    }
    pub mod crypto {
        pub mod multisig {
            pub mod v1beta1 {
                include!("prost/cosmos.crypto.multisig.v1beta1.rs");
            }
        }
    }
    pub mod authz {
        pub mod v1beta1 {
            include!("prost/cosmos.authz.v1beta1.rs");
        }
    }
    pub mod evidence {
        pub mod v1beta1 {
            include!("prost/cosmos.evidence.v1beta1.rs");
        }
    }
    pub mod vesting {
        pub mod v1beta1 {
            include!("prost/cosmos.vesting.v1beta1.rs");
        }
    }
    pub mod crypto {
        pub mod multisig {
            include!("prost/cosmos.crypto.multisig.rs");
        }
    }
    pub mod crypto {
        pub mod ed25519 {
            include!("prost/cosmos.crypto.ed25519.rs");
        }
    }
    pub mod base {
        pub mod reflection {
            pub mod v2alpha1 {
                include!("prost/cosmos.base.reflection.v2alpha1.rs");
            }
        }
    }
    pub mod auth {
        pub mod v1beta1 {
            include!("prost/cosmos.auth.v1beta1.rs");
        }
    }
    pub mod tx {
        pub mod signing {
            pub mod v1beta1 {
                include!("prost/cosmos.tx.signing.v1beta1.rs");
            }
        }
    }
}
pub mod google {
    pub mod api {
        include!("prost/google.api.rs");
    }
}
pub mod lbm {
    pub mod foundation {
        pub mod v1 {
            include!("prost/lbm.foundation.v1.rs");
        }
    }
    pub mod tx {
        pub mod v1beta1 {
            include!("prost/lbm.tx.v1beta1.rs");
        }
    }
    pub mod base {
        pub mod ostracon {
            pub mod v1 {
                include!("prost/lbm.base.ostracon.v1.rs");
            }
        }
    }
    pub mod bankplus {
        pub mod v1 {
            include!("prost/lbm.bankplus.v1.rs");
        }
    }
    pub mod stakingplus {
        pub mod v1 {
            include!("prost/lbm.stakingplus.v1.rs");
        }
    }
    pub mod collection {
        pub mod v1 {
            include!("prost/lbm.collection.v1.rs");
        }
    }
    pub mod token {
        pub mod v1 {
            include!("prost/lbm.token.v1.rs");
        }
    }
}
