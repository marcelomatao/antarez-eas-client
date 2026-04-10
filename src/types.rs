//! Core data types for EAS attestations and schemas.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// An on-chain attestation record returned by EAS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attestation {
    /// Unique attestation identifier (bytes32 hex, 0x-prefixed).
    pub uid: String,

    /// Schema UID this attestation belongs to.
    pub schema_uid: String,

    /// The encoded attestation data (hex).
    pub data: Vec<u8>,

    /// Address of the attester.
    pub attester: String,

    /// Recipient address (zero address if none).
    pub recipient: String,

    /// Whether this attestation is revocable.
    pub revocable: bool,

    /// Whether this attestation has been revoked.
    pub revoked: bool,

    /// On-chain timestamp (Unix seconds).
    pub timestamp: u64,

    /// Expiration time (0 = no expiration).
    pub expiration_time: u64,

    /// Transaction hash that created this attestation.
    pub transaction_hash: String,
}

/// Request to create a new attestation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationRequest {
    /// Schema UID to attest against (bytes32 hex, 0x-prefixed).
    pub schema_uid: String,

    /// The data to attest (raw bytes, will be ABI-encoded).
    pub data: Vec<u8>,

    /// Recipient address (use zero address for no specific recipient).
    pub recipient: String,

    /// Whether this attestation can be revoked later.
    pub revocable: bool,

    /// Expiration time in Unix seconds (0 = no expiration).
    pub expiration_time: u64,

    /// Reference UID to link to another attestation (zero bytes32 if none).
    pub ref_uid: String,
}

impl AttestationRequest {
    /// Creates a simple non-revocable attestation request with no recipient or expiration.
    pub fn simple(schema_uid: impl Into<String>, data: Vec<u8>) -> Self {
        Self {
            schema_uid: schema_uid.into(),
            data,
            recipient: "0x0000000000000000000000000000000000000000".to_string(),
            revocable: false,
            expiration_time: 0,
            ref_uid: "0x".to_string() + &"0".repeat(64),
        }
    }
}

/// An EAS schema record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaRecord {
    /// Schema UID (bytes32 hex, 0x-prefixed).
    pub uid: String,

    /// Schema string definition (e.g., "bytes32 merkleRoot").
    pub schema: String,

    /// Resolver contract address (zero address if none).
    pub resolver: String,

    /// Whether attestations under this schema are revocable.
    pub revocable: bool,
}

/// Request to register a new schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaRequest {
    /// Schema string definition (e.g., "bytes32 merkleRoot, uint64 timestamp").
    pub schema: String,

    /// Resolver contract address (zero address if none).
    pub resolver: String,

    /// Whether attestations under this schema should be revocable.
    pub revocable: bool,
}

/// Result of a batch attestation operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchAttestationResult {
    /// UIDs of the created attestations.
    pub uids: Vec<String>,

    /// Transaction hash for the batch.
    pub transaction_hash: String,

    /// Timestamp of the batch operation.
    pub timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_attestation_request() {
        let req = AttestationRequest::simple("0xabc123", vec![1, 2, 3]);
        assert_eq!(req.schema_uid, "0xabc123");
        assert!(!req.revocable);
        assert_eq!(req.expiration_time, 0);
        assert!(req.recipient.starts_with("0x000"));
    }

    #[test]
    fn test_attestation_serialization() {
        let attestation = Attestation {
            uid: "0x123".to_string(),
            schema_uid: "0xabc".to_string(),
            data: vec![1, 2, 3],
            attester: "0xdead".to_string(),
            recipient: "0x0000000000000000000000000000000000000000".to_string(),
            revocable: false,
            revoked: false,
            timestamp: 1700000000,
            expiration_time: 0,
            transaction_hash: "0xtxhash".to_string(),
        };
        let json = serde_json::to_string(&attestation).unwrap();
        let restored: Attestation = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.uid, attestation.uid);
        assert_eq!(restored.timestamp, attestation.timestamp);
    }
}
