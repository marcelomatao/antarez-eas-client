//! ABI encoding and decoding for EAS attestation data.
//!
//! EAS schemas define the structure of attestation data as Solidity types.
//! This module encodes inputs into ABI-encoded bytes and decodes them back.

use alloy::sol;
use alloy::sol_types::SolValue;

use crate::error::{EasError, EncodingError};

// Define the base attestation data layout.
// Schema: "bytes32 dataHash, string description, uint64 timestamp"
sol! {
    /// Standard attestation payload for generic data hashes.
    struct AttestationData {
        bytes32 dataHash;
        string description;
        uint64 timestamp;
    }

    /// Minimal attestation payload with just a hash.
    struct SimpleAttestationData {
        bytes32 dataHash;
    }
}

/// Encode a full attestation payload.
///
/// # Arguments
/// * `data_hash` — 32-byte hash (hex string, 0x-prefixed or raw).
/// * `description` — Human-readable description of what was attested.
/// * `timestamp` — Unix timestamp in seconds.
///
/// # Errors
/// Returns `EasError::Encoding` if the hash is not exactly 32 bytes.
pub fn encode_attestation(
    data_hash: &str,
    description: &str,
    timestamp: u64,
) -> Result<Vec<u8>, EasError> {
    let hash_bytes = parse_bytes32(data_hash)?;

    let payload = AttestationData {
        dataHash: hash_bytes.into(),
        description: description.to_string(),
        timestamp,
    };

    Ok(payload.abi_encode())
}

/// Decode an ABI-encoded full attestation payload.
///
/// # Errors
/// Returns `EasError::Encoding` if decoding fails.
pub fn decode_attestation(data: &[u8]) -> Result<(String, String, u64), EasError> {
    let decoded = AttestationData::abi_decode(data)
        .map_err(|e| EasError::Encoding(EncodingError::AbiDecodingFailed {
            details: e.to_string(),
        }))?;

    let hash_hex = format!("0x{}", hex::encode(decoded.dataHash));
    Ok((hash_hex, decoded.description, decoded.timestamp))
}

/// Encode a simple attestation with just a data hash.
pub fn encode_simple(data_hash: &str) -> Result<Vec<u8>, EasError> {
    let hash_bytes = parse_bytes32(data_hash)?;

    let payload = SimpleAttestationData {
        dataHash: hash_bytes.into(),
    };

    Ok(payload.abi_encode())
}

/// Decode a simple attestation (data hash only).
pub fn decode_simple(data: &[u8]) -> Result<String, EasError> {
    let decoded = SimpleAttestationData::abi_decode(data)
        .map_err(|e| EasError::Encoding(EncodingError::AbiDecodingFailed {
            details: e.to_string(),
        }))?;

    Ok(format!("0x{}", hex::encode(decoded.dataHash)))
}

/// Parse a hex string into exactly 32 bytes.
fn parse_bytes32(hex_str: &str) -> Result<[u8; 32], EasError> {
    let stripped = hex_str.strip_prefix("0x").unwrap_or(hex_str);
    let bytes = hex::decode(stripped)
        .map_err(|e| EasError::Encoding(EncodingError::InvalidHex {
            details: e.to_string(),
        }))?;

    let actual_len = bytes.len();
    bytes
        .try_into()
        .map_err(|_| EasError::Encoding(EncodingError::InvalidDataLength {
            expected: 32,
            actual: actual_len,
        }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_hash() -> String {
        format!("0x{}", "ab".repeat(32))
    }

    #[test]
    fn test_encode_decode_full_roundtrip() {
        let hash = sample_hash();
        let desc = "test attestation";
        let ts = 1700000000u64;

        let encoded = encode_attestation(&hash, desc, ts).unwrap();
        let (decoded_hash, decoded_desc, decoded_ts) = decode_attestation(&encoded).unwrap();

        assert_eq!(decoded_hash, hash);
        assert_eq!(decoded_desc, desc);
        assert_eq!(decoded_ts, ts);
    }

    #[test]
    fn test_encode_decode_simple_roundtrip() {
        let hash = sample_hash();
        let encoded = encode_simple(&hash).unwrap();
        let decoded = decode_simple(&encoded).unwrap();
        assert_eq!(decoded, hash);
    }

    #[test]
    fn test_invalid_hex() {
        let result = encode_simple("0xZZZZ");
        assert!(result.is_err());
    }

    #[test]
    fn test_wrong_length() {
        let short = "0xabcd"; // only 2 bytes
        let result = encode_simple(short);
        assert!(result.is_err());
    }

    #[test]
    fn test_no_prefix() {
        let hash = "ab".repeat(32);
        let encoded = encode_simple(&hash).unwrap();
        let decoded = decode_simple(&encoded).unwrap();
        assert_eq!(decoded, format!("0x{hash}"));
    }
}
