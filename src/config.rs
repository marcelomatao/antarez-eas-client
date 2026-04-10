//! EAS client configuration.

use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Configuration for the EAS client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EasConfig {
    /// Ethereum JSON-RPC endpoint URL.
    pub rpc_url: String,

    /// EAS contract address (0x-prefixed hex).
    pub eas_contract_address: String,

    /// Schema Registry contract address (0x-prefixed hex).
    pub schema_registry_address: String,

    /// Expected chain ID (for safety checks).
    pub chain_id: u64,

    /// Transaction confirmation timeout in seconds.
    #[serde(default = "default_tx_timeout_secs")]
    pub tx_timeout_secs: u64,

    /// Number of block confirmations to wait for.
    #[serde(default = "default_confirmations")]
    pub confirmations: u64,
}

fn default_tx_timeout_secs() -> u64 {
    60
}

fn default_confirmations() -> u64 {
    1
}

impl EasConfig {
    /// Creates config for a known chain using predefined contract addresses.
    pub fn for_chain(chain: &crate::chain::ChainConfig, rpc_url: impl Into<String>) -> Self {
        Self {
            rpc_url: rpc_url.into(),
            eas_contract_address: chain.eas_address.to_string(),
            schema_registry_address: chain.schema_registry_address.to_string(),
            chain_id: chain.chain_id,
            tx_timeout_secs: default_tx_timeout_secs(),
            confirmations: default_confirmations(),
        }
    }

    /// Returns the transaction timeout as a `Duration`.
    pub fn tx_timeout(&self) -> Duration {
        Duration::from_secs(self.tx_timeout_secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        assert_eq!(default_tx_timeout_secs(), 60);
        assert_eq!(default_confirmations(), 1);
    }

    #[test]
    fn test_tx_timeout_duration() {
        let config = EasConfig {
            rpc_url: "http://localhost:8545".to_string(),
            eas_contract_address: "0x123".to_string(),
            schema_registry_address: "0x456".to_string(),
            chain_id: 1,
            tx_timeout_secs: 30,
            confirmations: 2,
        };
        assert_eq!(config.tx_timeout(), Duration::from_secs(30));
    }
}
