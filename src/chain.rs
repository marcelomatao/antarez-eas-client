//! Known chain configurations with EAS contract addresses.
//!
//! EAS is deployed at the same addresses across all supported chains.
//! See: https://docs.attest.org/docs/quick--start/contracts

/// Chain configuration with EAS contract addresses.
#[derive(Debug, Clone)]
pub struct ChainConfig {
    /// Human-readable chain name.
    pub name: &'static str,

    /// Chain ID.
    pub chain_id: u64,

    /// EAS contract address.
    pub eas_address: &'static str,

    /// Schema Registry contract address.
    pub schema_registry_address: &'static str,
}

/// EAS v1.2.0 contract addresses (same on all supported chains).
const EAS_ADDRESS: &str = "0xA1207F3BBa224E2c9c3c6D5aF63D816e52E89119";
const SCHEMA_REGISTRY_ADDRESS: &str = "0xA7b39296258348C78294F95B872b282326A97BDF";

/// Ethereum Mainnet.
pub const ETHEREUM_MAINNET: ChainConfig = ChainConfig {
    name: "Ethereum Mainnet",
    chain_id: 1,
    eas_address: EAS_ADDRESS,
    schema_registry_address: SCHEMA_REGISTRY_ADDRESS,
};

/// Arbitrum One (L2).
pub const ARBITRUM_ONE: ChainConfig = ChainConfig {
    name: "Arbitrum One",
    chain_id: 42161,
    eas_address: "0xbD75f629A22Dc1ceD33dDA0b68c546A1c035c458",
    schema_registry_address: "0xA310da9c5B885E7fb3fbA9D66E9Ba6Df512b78eB",
};

/// Base (L2).
pub const BASE: ChainConfig = ChainConfig {
    name: "Base",
    chain_id: 8453,
    eas_address: "0x4200000000000000000000000000000000000021",
    schema_registry_address: "0x4200000000000000000000000000000000000020",
};

/// Optimism (L2).
pub const OPTIMISM: ChainConfig = ChainConfig {
    name: "Optimism",
    chain_id: 10,
    eas_address: "0x4200000000000000000000000000000000000021",
    schema_registry_address: "0x4200000000000000000000000000000000000020",
};

/// Ethereum Sepolia (testnet).
pub const SEPOLIA: ChainConfig = ChainConfig {
    name: "Sepolia",
    chain_id: 11155111,
    eas_address: "0xC2679fBD37d54388Ce493F1DB75320D236e1815e",
    schema_registry_address: "0x0a7E2Ff54e76B8E6659aedc9103FB21c038050D0",
};

/// Look up a chain config by chain ID. Returns `None` for unsupported chains.
pub fn chain_by_id(chain_id: u64) -> Option<&'static ChainConfig> {
    match chain_id {
        1 => Some(&ETHEREUM_MAINNET),
        42161 => Some(&ARBITRUM_ONE),
        8453 => Some(&BASE),
        10 => Some(&OPTIMISM),
        11155111 => Some(&SEPOLIA),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_lookup() {
        let chain = chain_by_id(42161).unwrap();
        assert_eq!(chain.name, "Arbitrum One");
        assert_eq!(chain.chain_id, 42161);

        assert!(chain_by_id(999999).is_none());
    }

    #[test]
    fn test_known_chains() {
        assert_eq!(ETHEREUM_MAINNET.chain_id, 1);
        assert_eq!(ARBITRUM_ONE.chain_id, 42161);
        assert_eq!(BASE.chain_id, 8453);
        assert_eq!(OPTIMISM.chain_id, 10);
        assert_eq!(SEPOLIA.chain_id, 11155111);
    }

    #[test]
    fn test_addresses_format() {
        assert!(ETHEREUM_MAINNET.eas_address.starts_with("0x"));
        assert_eq!(ETHEREUM_MAINNET.eas_address.len(), 42);
    }
}
