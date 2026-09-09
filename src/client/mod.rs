//! EAS client core — provider, signer, and shared client logic.

pub mod attestation;
pub mod query;
pub mod schema;

use std::sync::Arc;

use alloy::network::{Ethereum, EthereumWallet};
use alloy::primitives::Address;
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;

use crate::config::EasConfig;
use crate::error::{EasError, ProviderError};

/// The main EAS client.
///
/// Holds a configured alloy provider and the contract addresses needed
/// for attestation and schema operations. `Clone` (the provider is a
/// cheap `Arc` handle) so callers can share one client across tasks —
/// e.g. an `Arc<EasClient>` behind application state.
#[derive(Clone)]
pub struct EasClient {
    /// Provider: signed when built via [`EasClient::new`], account-less
    /// when built via [`EasClient::read_only`]. Reads work either way;
    /// a write on a read-only client fails at send time (no signer).
    pub(crate) provider: Arc<DynProvider<Ethereum>>,

    /// EAS contract address.
    pub(crate) eas_address: Address,

    /// Schema Registry contract address.
    pub(crate) schema_registry_address: Address,

    /// Expected chain ID.
    pub(crate) chain_id: u64,

    /// Transaction confirmation count.
    pub(crate) confirmations: u64,
}

impl EasClient {
    /// Create a new EAS client from config and a private key.
    ///
    /// # Arguments
    /// * `config` — EAS configuration (RPC URL, contract addresses, chain ID).
    /// * `private_key` — Hex-encoded private key (with or without 0x prefix).
    ///
    /// # Errors
    /// Returns `EasError::Config` if the private key or addresses are invalid.
    /// Returns `EasError::Provider` if the RPC is unreachable or chain ID doesn't match.
    pub async fn new(config: &EasConfig, private_key: &str) -> Result<Self, EasError> {
        // Parse signer
        let stripped = private_key.strip_prefix("0x").unwrap_or(private_key);
        let signer: PrivateKeySigner = stripped
            .parse()
            .map_err(|e| EasError::Config {
                message: format!("invalid private key: {e}"),
            })?;

        let wallet = EthereumWallet::from(signer);

        // Build provider with signer
        let rpc_url: reqwest::Url = config
            .rpc_url
            .parse()
            .map_err(|e| EasError::Config {
                message: format!("invalid RPC URL: {e}"),
            })?;

        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect_http(rpc_url)
            .erased();

        // Verify chain ID
        let actual_chain_id = provider
            .get_chain_id()
            .await
            .map_err(|e| {
                EasError::Provider(ProviderError::RequestFailed {
                    details: format!("failed to get chain ID: {e}"),
                })
            })?;

        if actual_chain_id != config.chain_id {
            return Err(EasError::Provider(ProviderError::ChainIdMismatch {
                expected: config.chain_id,
                actual: actual_chain_id,
            }));
        }

        // Parse addresses
        let eas_address: Address = config
            .eas_contract_address
            .parse()
            .map_err(|e| EasError::Config {
                message: format!("invalid EAS contract address: {e}"),
            })?;

        let schema_registry_address: Address = config
            .schema_registry_address
            .parse()
            .map_err(|e| EasError::Config {
                message: format!("invalid schema registry address: {e}"),
            })?;

        Ok(Self {
            provider: Arc::new(provider),
            eas_address,
            schema_registry_address,
            chain_id: config.chain_id,
            confirmations: config.confirmations,
        })
    }

    /// Create a read-only EAS client — no signing key, no transaction
    /// capability. Reads (`query_attestation`, `is_valid`,
    /// `get_attestations`) work exactly as on a signed client; any write
    /// attempt fails at send time because the underlying provider carries
    /// no wallet.
    ///
    /// Use this for verification paths that must not hold the on-chain
    /// signing key: a read-only consumer cannot sign anything even if
    /// compromised.
    pub async fn read_only(config: &EasConfig) -> Result<Self, EasError> {
        let rpc_url: reqwest::Url = config
            .rpc_url
            .parse()
            .map_err(|e| EasError::Config {
                message: format!("invalid RPC URL: {e}"),
            })?;

        let provider = ProviderBuilder::new()
            .connect_http(rpc_url)
            .erased();

        let actual_chain_id = provider
            .get_chain_id()
            .await
            .map_err(|e| {
                EasError::Provider(ProviderError::RequestFailed {
                    details: format!("failed to get chain ID: {e}"),
                })
            })?;

        if actual_chain_id != config.chain_id {
            return Err(EasError::Provider(ProviderError::ChainIdMismatch {
                expected: config.chain_id,
                actual: actual_chain_id,
            }));
        }

        let eas_address: Address = config
            .eas_contract_address
            .parse()
            .map_err(|e| EasError::Config {
                message: format!("invalid EAS contract address: {e}"),
            })?;

        let schema_registry_address: Address = config
            .schema_registry_address
            .parse()
            .map_err(|e| EasError::Config {
                message: format!("invalid schema registry address: {e}"),
            })?;

        Ok(Self {
            provider: Arc::new(provider),
            eas_address,
            schema_registry_address,
            chain_id: config.chain_id,
            confirmations: config.confirmations,
        })
    }

    /// Returns the EAS contract address.
    pub fn eas_address(&self) -> Address {
        self.eas_address
    }

    /// Returns the chain ID.
    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }
}
