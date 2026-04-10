//! antarez-eas-client — Generic async Rust client for the Ethereum Attestation Service (EAS)
//!
//! Provides typed interfaces for creating, querying, and verifying on-chain attestations.
//! Domain-agnostic — usable by any application that needs to attest data on EAS-supported chains.

pub mod chain;
pub mod client;
pub mod config;
pub mod encoding;
pub mod error;
pub mod types;

pub use chain::{ChainConfig, chain_by_id};
pub use client::EasClient;
pub use config::EasConfig;
pub use encoding::{decode_attestation, decode_simple, encode_attestation, encode_simple};
pub use error::EasError;
pub use types::{Attestation, AttestationRequest, BatchAttestationResult, SchemaRecord, SchemaRequest};

/// Returns the crate version.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let v = version();
        assert!(!v.is_empty());
        assert!(v.contains('.'));
    }
}
