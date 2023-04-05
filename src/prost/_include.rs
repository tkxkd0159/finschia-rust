pub mod cosmos {
    pub mod auth {
        pub mod v1beta1 {
            include!("cosmos.auth.v1beta1.rs");
        }
    }
    pub mod authz {
        pub mod v1beta1 {
            include!("cosmos.authz.v1beta1.rs");
        }
    }
    pub mod bank {
        pub mod v1beta1 {
            include!("cosmos.bank.v1beta1.rs");
        }
    }
    pub mod base {
        pub mod abci {
            pub mod v1beta1 {
                include!("cosmos.base.abci.v1beta1.rs");
            }
        }
        pub mod kv {
            pub mod v1beta1 {
                include!("cosmos.base.kv.v1beta1.rs");
            }
        }
        pub mod node {
            pub mod v1beta1 {
                include!("cosmos.base.node.v1beta1.rs");
            }
        }
        pub mod query {
            pub mod v1beta1 {
                include!("cosmos.base.query.v1beta1.rs");
            }
        }
        pub mod reflection {
            pub mod v1beta1 {
                include!("cosmos.base.reflection.v1beta1.rs");
            }
            pub mod v2alpha1 {
                include!("cosmos.base.reflection.v2alpha1.rs");
            }
        }
        pub mod snapshots {
            pub mod v1beta1 {
                include!("cosmos.base.snapshots.v1beta1.rs");
            }
        }
        pub mod store {
            pub mod v1beta1 {
                include!("cosmos.base.store.v1beta1.rs");
            }
        }
        pub mod v1beta1 {
            include!("cosmos.base.v1beta1.rs");
        }
    }
    pub mod capability {
        pub mod v1beta1 {
            include!("cosmos.capability.v1beta1.rs");
        }
    }
    pub mod crisis {
        pub mod v1beta1 {
            include!("cosmos.crisis.v1beta1.rs");
        }
    }
    pub mod crypto {
        pub mod ed25519 {
            include!("cosmos.crypto.ed25519.rs");
        }
        pub mod multisig {
            pub mod v1beta1 {
                include!("cosmos.crypto.multisig.v1beta1.rs");
            }
            include!("cosmos.crypto.multisig.rs");
        }
        pub mod secp256k1 {
            include!("cosmos.crypto.secp256k1.rs");
        }
        pub mod secp256r1 {
            include!("cosmos.crypto.secp256r1.rs");
        }
    }
    pub mod distribution {
        pub mod v1beta1 {
            include!("cosmos.distribution.v1beta1.rs");
        }
    }
    pub mod evidence {
        pub mod v1beta1 {
            include!("cosmos.evidence.v1beta1.rs");
        }
    }
    pub mod feegrant {
        pub mod v1beta1 {
            include!("cosmos.feegrant.v1beta1.rs");
        }
    }
    pub mod genutil {
        pub mod v1beta1 {
            include!("cosmos.genutil.v1beta1.rs");
        }
    }
    pub mod gov {
        pub mod v1beta1 {
            include!("cosmos.gov.v1beta1.rs");
        }
    }
    pub mod mint {
        pub mod v1beta1 {
            include!("cosmos.mint.v1beta1.rs");
        }
    }
    pub mod params {
        pub mod v1beta1 {
            include!("cosmos.params.v1beta1.rs");
        }
    }
    pub mod slashing {
        pub mod v1beta1 {
            include!("cosmos.slashing.v1beta1.rs");
        }
    }
    pub mod staking {
        pub mod v1beta1 {
            include!("cosmos.staking.v1beta1.rs");
        }
    }
    pub mod tx {
        pub mod signing {
            pub mod v1beta1 {
                include!("cosmos.tx.signing.v1beta1.rs");
            }
        }
        pub mod v1beta1 {
            include!("cosmos.tx.v1beta1.rs");
        }
    }
    pub mod upgrade {
        pub mod v1beta1 {
            include!("cosmos.upgrade.v1beta1.rs");
        }
    }
    pub mod vesting {
        pub mod v1beta1 {
            include!("cosmos.vesting.v1beta1.rs");
        }
    }
}
pub mod google {
    pub mod api {
        include!("google.api.rs");
    }
}
pub mod ics23 {
    include!("ics23.rs");
}
pub mod lbm {
    pub mod bankplus {
        pub mod v1 {
            include!("lbm.bankplus.v1.rs");
        }
    }
    pub mod base {
        pub mod ostracon {
            pub mod v1 {
                include!("lbm.base.ostracon.v1.rs");
            }
        }
    }
    pub mod collection {
        pub mod v1 {
            include!("lbm.collection.v1.rs");
        }
    }
    pub mod foundation {
        pub mod v1 {
            include!("lbm.foundation.v1.rs");
        }
    }
    pub mod stakingplus {
        pub mod v1 {
            include!("lbm.stakingplus.v1.rs");
        }
    }
    pub mod token {
        pub mod v1 {
            include!("lbm.token.v1.rs");
        }
    }
    pub mod tx {
        pub mod v1beta1 {
            include!("lbm.tx.v1beta1.rs");
        }
    }
}
pub mod ostracon {
    pub mod abci {
        include!("ostracon.abci.rs");
    }
    pub mod types {
        include!("ostracon.types.rs");
    }
}
pub mod tendermint {
    pub mod abci {
        include!("tendermint.abci.rs");
    }
    pub mod blockchain {
        include!("tendermint.blockchain.rs");
    }
    pub mod consensus {
        include!("tendermint.consensus.rs");
    }
    pub mod crypto {
        include!("tendermint.crypto.rs");
    }
    pub mod libs {
        pub mod bits {
            include!("tendermint.libs.bits.rs");
        }
    }
    pub mod mempool {
        include!("tendermint.mempool.rs");
    }
    pub mod p2p {
        include!("tendermint.p2p.rs");
    }
    pub mod privval {
        include!("tendermint.privval.rs");
    }
    pub mod state {
        include!("tendermint.state.rs");
    }
    pub mod statesync {
        include!("tendermint.statesync.rs");
    }
    pub mod store {
        include!("tendermint.store.rs");
    }
    pub mod types {
        include!("tendermint.types.rs");
    }
    pub mod version {
        include!("tendermint.version.rs");
    }
}
