//! Error types for the EAS client library.

use thiserror::Error;

/// Top-level error type for all EAS operations.
#[derive(Debug, Error)]
pub enum EasError {
    #[error("Provider error: {0}")]
    Provider(#[from] ProviderError),

    #[error("Contract error: {0}")]
    Contract(#[from] ContractError),

    #[error("Encoding error: {0}")]
    Encoding(#[from] EncodingError),

    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Schema error: {0}")]
    Schema(#[from] SchemaError),
}

/// Errors from the Ethereum provider (RPC communication).
#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("RPC connection failed: {url}")]
    ConnectionFailed { url: String },

    #[error("RPC request failed: {details}")]
    RequestFailed { details: String },

    #[error("chain ID mismatch: expected {expected}, got {actual}")]
    ChainIdMismatch { expected: u64, actual: u64 },

    #[error("transaction not found: {tx_hash}")]
    TransactionNotFound { tx_hash: String },

    #[error("transaction reverted: {reason}")]
    TransactionReverted { reason: String },

    #[error("RPC timeout after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },
}

/// Errors from EAS smart contract interactions.
#[derive(Debug, Error)]
pub enum ContractError {
    #[error("attestation not found: {uid}")]
    AttestationNotFound { uid: String },

    #[error("attestation revoked: {uid}")]
    AttestationRevoked { uid: String },

    #[error("invalid attestation UID: {uid}")]
    InvalidUid { uid: String },

    #[error("no receipt returned for transaction")]
    NoReceipt,

    #[error("failed to extract UID from transaction logs")]
    UidExtractionFailed,

    #[error("contract call failed: {details}")]
    CallFailed { details: String },

    #[error("transaction failed: {details}")]
    TransactionFailed { details: String },

    #[error("empty batch: at least one request is required")]
    EmptyBatch,

    #[error("insufficient funds for transaction: need {needed}, have {balance}")]
    InsufficientFunds { needed: String, balance: String },
}

/// Errors from ABI encoding/decoding operations.
#[derive(Debug, Error)]
pub enum EncodingError {
    #[error("ABI encoding failed: {details}")]
    AbiEncodingFailed { details: String },

    #[error("ABI decoding failed: {details}")]
    AbiDecodingFailed { details: String },

    #[error("invalid hex string: {details}")]
    InvalidHex { details: String },

    #[error("invalid data length: expected {expected} bytes, got {actual}")]
    InvalidDataLength { expected: usize, actual: usize },
}

/// Errors related to EAS schema operations.
#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("schema not found: {uid}")]
    SchemaNotFound { uid: String },

    #[error("invalid schema string: {schema}")]
    InvalidSchema { schema: String },

    #[error("schema registration failed: {reason}")]
    RegistrationFailed { reason: String },

    #[error("invalid schema UID format: {uid}")]
    InvalidSchemaUid { uid: String },
}

/// Numeric error codes for programmatic handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    // Provider errors: 1xxx
    ConnectionFailed = 1001,
    RequestFailed = 1002,
    ChainIdMismatch = 1003,
    TransactionNotFound = 1004,
    TransactionReverted = 1005,
    Timeout = 1006,

    // Contract errors: 2xxx
    AttestationNotFound = 2001,
    AttestationRevoked = 2002,
    InvalidUid = 2003,
    NoReceipt = 2004,
    UidExtractionFailed = 2005,
    CallFailed = 2006,
    TransactionFailed = 2007,
    EmptyBatch = 2008,
    InsufficientFunds = 2009,

    // Encoding errors: 3xxx
    AbiEncodingFailed = 3001,
    AbiDecodingFailed = 3002,
    InvalidHex = 3003,
    InvalidDataLength = 3004,

    // Schema errors: 4xxx
    SchemaNotFound = 4001,
    InvalidSchema = 4002,
    RegistrationFailed = 4003,
    InvalidSchemaUid = 4004,

    // Config errors: 9xxx
    ConfigError = 9001,
}

impl EasError {
    /// Returns the numeric error code for this error.
    pub fn code(&self) -> ErrorCode {
        match self {
            EasError::Provider(e) => match e {
                ProviderError::ConnectionFailed { .. } => ErrorCode::ConnectionFailed,
                ProviderError::RequestFailed { .. } => ErrorCode::RequestFailed,
                ProviderError::ChainIdMismatch { .. } => ErrorCode::ChainIdMismatch,
                ProviderError::TransactionNotFound { .. } => ErrorCode::TransactionNotFound,
                ProviderError::TransactionReverted { .. } => ErrorCode::TransactionReverted,
                ProviderError::Timeout { .. } => ErrorCode::Timeout,
            },
            EasError::Contract(e) => match e {
                ContractError::AttestationNotFound { .. } => ErrorCode::AttestationNotFound,
                ContractError::AttestationRevoked { .. } => ErrorCode::AttestationRevoked,
                ContractError::InvalidUid { .. } => ErrorCode::InvalidUid,
                ContractError::NoReceipt => ErrorCode::NoReceipt,
                ContractError::UidExtractionFailed => ErrorCode::UidExtractionFailed,
                ContractError::CallFailed { .. } => ErrorCode::CallFailed,
                ContractError::TransactionFailed { .. } => ErrorCode::TransactionFailed,
                ContractError::EmptyBatch => ErrorCode::EmptyBatch,
                ContractError::InsufficientFunds { .. } => ErrorCode::InsufficientFunds,
            },
            EasError::Encoding(e) => match e {
                EncodingError::AbiEncodingFailed { .. } => ErrorCode::AbiEncodingFailed,
                EncodingError::AbiDecodingFailed { .. } => ErrorCode::AbiDecodingFailed,
                EncodingError::InvalidHex { .. } => ErrorCode::InvalidHex,
                EncodingError::InvalidDataLength { .. } => ErrorCode::InvalidDataLength,
            },
            EasError::Schema(e) => match e {
                SchemaError::SchemaNotFound { .. } => ErrorCode::SchemaNotFound,
                SchemaError::InvalidSchema { .. } => ErrorCode::InvalidSchema,
                SchemaError::RegistrationFailed { .. } => ErrorCode::RegistrationFailed,
                SchemaError::InvalidSchemaUid { .. } => ErrorCode::InvalidSchemaUid,
            },
            EasError::Config { .. } => ErrorCode::ConfigError,
        }
    }

    /// Whether this error is transient and the operation can be retried.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            EasError::Provider(ProviderError::ConnectionFailed { .. })
                | EasError::Provider(ProviderError::RequestFailed { .. })
                | EasError::Provider(ProviderError::Timeout { .. })
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        let err = EasError::Provider(ProviderError::ConnectionFailed {
            url: "http://localhost:8545".to_string(),
        });
        assert_eq!(err.code(), ErrorCode::ConnectionFailed);
    }

    #[test]
    fn test_retryable() {
        let retryable = EasError::Provider(ProviderError::Timeout { timeout_ms: 5000 });
        assert!(retryable.is_retryable());

        let not_retryable = EasError::Contract(ContractError::AttestationNotFound {
            uid: "0x123".to_string(),
        });
        assert!(!not_retryable.is_retryable());
    }

    #[test]
    fn test_error_display() {
        let err = EasError::Contract(ContractError::AttestationNotFound {
            uid: "0xabc".to_string(),
        });
        let msg = format!("{}", err);
        assert!(msg.contains("attestation not found"));
        assert!(msg.contains("0xabc"));
    }

    #[test]
    fn test_encoding_error() {
        let err = EasError::Encoding(EncodingError::InvalidHex {
            details: "odd length".to_string(),
        });
        assert_eq!(err.code(), ErrorCode::InvalidHex);
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_schema_error() {
        let err = EasError::Schema(SchemaError::InvalidSchemaUid {
            uid: "not-a-uid".to_string(),
        });
        assert_eq!(err.code(), ErrorCode::InvalidSchemaUid);
    }
}
