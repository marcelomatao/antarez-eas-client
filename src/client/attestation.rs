//! Attestation operations — create, get, and batch attest.

use alloy::primitives::{Address, FixedBytes, U256};
use alloy::sol;

use crate::error::{ContractError, EasError};
use crate::types::{Attestation, AttestationRequest, BatchAttestationResult};

use super::EasClient;

// Generate type-safe bindings for the EAS contract.
sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    interface IEAS {
        struct AttestationRequestData {
            address recipient;
            uint64 expirationTime;
            bool revocable;
            bytes32 refUID;
            bytes data;
            uint256 value;
        }

        struct EASAttestationRequest {
            bytes32 schema;
            AttestationRequestData data;
        }

        struct MultiAttestationRequest {
            bytes32 schema;
            AttestationRequestData[] data;
        }

        struct EASAttestation {
            bytes32 uid;
            bytes32 schema;
            uint64 time;
            uint64 expirationTime;
            uint64 revocationTime;
            bytes32 refUID;
            address recipient;
            address attester;
            bool revocable;
            bytes data;
        }

        function attest(EASAttestationRequest calldata request) external payable returns (bytes32);
        function multiAttest(MultiAttestationRequest[] calldata multiRequests) external payable returns (bytes32[]);
        function getAttestation(bytes32 uid) external view returns (EASAttestation memory);
    }
}

impl EasClient {
    /// Create a single on-chain attestation.
    ///
    /// Sends a transaction to the EAS contract and waits for confirmation.
    /// Returns the attestation record with its UID and transaction hash.
    pub async fn create_attestation(
        &self,
        request: &AttestationRequest,
    ) -> Result<Attestation, EasError> {
        let schema_uid = parse_bytes32(&request.schema_uid)?;
        let recipient = parse_address(&request.recipient)?;
        let ref_uid = parse_bytes32(&request.ref_uid)?;

        let eas = IEAS::new(self.eas_address, &*self.provider);

        let call = eas.attest(IEAS::EASAttestationRequest {
            schema: schema_uid,
            data: IEAS::AttestationRequestData {
                recipient,
                expirationTime: request.expiration_time,
                revocable: request.revocable,
                refUID: ref_uid,
                data: request.data.clone().into(),
                value: U256::ZERO,
            },
        });

        let pending = call
            .send()
            .await
            .map_err(|e| {
                EasError::Contract(ContractError::TransactionFailed {
                    details: e.to_string(),
                })
            })?;

        let receipt = pending
            .with_required_confirmations(self.confirmations)
            .get_receipt()
            .await
            .map_err(|e| {
                EasError::Contract(ContractError::TransactionFailed {
                    details: e.to_string(),
                })
            })?;

        let tx_hash = format!("0x{}", hex::encode(receipt.transaction_hash));

        // Extract the attestation UID from the Attested event log.
        let attestation_uid = extract_uid_from_receipt(&receipt)?;

        Ok(Attestation {
            uid: format!("0x{}", hex::encode(attestation_uid)),
            schema_uid: request.schema_uid.clone(),
            data: request.data.clone(),
            attester: String::new(), // filled by caller if needed
            recipient: request.recipient.clone(),
            revocable: request.revocable,
            revoked: false,
            timestamp: chrono::Utc::now().timestamp() as u64,
            expiration_time: request.expiration_time,
            transaction_hash: tx_hash,
        })
    }

    /// Retrieve an existing attestation by UID.
    pub async fn get_attestation(&self, uid: &str) -> Result<Attestation, EasError> {
        let uid_bytes = parse_bytes32(uid)?;
        let eas = IEAS::new(self.eas_address, &*self.provider);

        let result = eas
            .getAttestation(uid_bytes)
            .call()
            .await
            .map_err(|e| {
                EasError::Contract(ContractError::CallFailed {
                    details: e.to_string(),
                })
            })?;

        let att = result;

        Ok(Attestation {
            uid: format!("0x{}", hex::encode(att.uid)),
            schema_uid: format!("0x{}", hex::encode(att.schema)),
            data: att.data.to_vec(),
            attester: format!("{:?}", att.attester),
            recipient: format!("{:?}", att.recipient),
            revocable: att.revocable,
            revoked: att.revocationTime > 0,
            timestamp: att.time,
            expiration_time: att.expirationTime,
            transaction_hash: String::new(),
        })
    }

    /// Create multiple attestations for the same schema in a single transaction.
    pub async fn batch_attest(
        &self,
        schema_uid: &str,
        requests: &[AttestationRequest],
    ) -> Result<BatchAttestationResult, EasError> {
        if requests.is_empty() {
            return Err(EasError::Contract(ContractError::EmptyBatch));
        }

        let schema = parse_bytes32(schema_uid)?;
        let eas = IEAS::new(self.eas_address, &*self.provider);

        let data_items: Vec<IEAS::AttestationRequestData> = requests
            .iter()
            .map(|r| {
                Ok(IEAS::AttestationRequestData {
                    recipient: parse_address(&r.recipient)?,
                    expirationTime: r.expiration_time,
                    revocable: r.revocable,
                    refUID: parse_bytes32(&r.ref_uid)?,
                    data: r.data.clone().into(),
                    value: U256::ZERO,
                })
            })
            .collect::<Result<Vec<_>, EasError>>()?;

        let multi_req = IEAS::MultiAttestationRequest {
            schema,
            data: data_items,
        };

        let call = eas.multiAttest(vec![multi_req]);

        let pending = call
            .send()
            .await
            .map_err(|e| {
                EasError::Contract(ContractError::TransactionFailed {
                    details: e.to_string(),
                })
            })?;

        let receipt = pending
            .with_required_confirmations(self.confirmations)
            .get_receipt()
            .await
            .map_err(|e| {
                EasError::Contract(ContractError::TransactionFailed {
                    details: e.to_string(),
                })
            })?;

        let uids = extract_uids_from_receipt(&receipt)?;
        let tx_hash = format!("0x{}", hex::encode(receipt.transaction_hash));

        Ok(BatchAttestationResult {
            uids: uids.iter().map(|u| format!("0x{}", hex::encode(u))).collect(),
            transaction_hash: tx_hash,
            timestamp: chrono::Utc::now(),
        })
    }
}

// --- Helpers ---

fn parse_bytes32(hex_str: &str) -> Result<FixedBytes<32>, EasError> {
    let stripped = hex_str.strip_prefix("0x").unwrap_or(hex_str);
    let bytes = hex::decode(stripped).map_err(|e| {
        EasError::Contract(ContractError::CallFailed {
            details: format!("bad hex: {e}"),
        })
    })?;
    let arr: [u8; 32] = bytes.try_into().map_err(|_| {
        EasError::Contract(ContractError::CallFailed {
            details: "expected 32 bytes".into(),
        })
    })?;
    Ok(FixedBytes(arr))
}

fn parse_address(hex_str: &str) -> Result<Address, EasError> {
    hex_str
        .parse()
        .map_err(|e: alloy::hex::FromHexError| EasError::Config {
            message: format!("invalid address: {e}"),
        })
}

/// Extract attestation UID from transaction receipt logs (Attested event).
///
/// The EAS `Attested` event has 3 indexed topics (event sig, recipient, schemaUID)
/// and the UID as the first 32 bytes of the non-indexed data.
fn extract_uid_from_receipt(
    receipt: &alloy::rpc::types::TransactionReceipt,
) -> Result<[u8; 32], EasError> {
    for log in receipt.inner.logs() {
        if log.topics().len() >= 2 && log.data().data.len() >= 32 {
            let uid: [u8; 32] = log.data().data[..32]
                .try_into()
                .map_err(|_| EasError::Contract(ContractError::UidExtractionFailed))?;
            return Ok(uid);
        }
    }
    Err(EasError::Contract(ContractError::UidExtractionFailed))
}

/// Extract multiple UIDs from a batch attestation receipt.
fn extract_uids_from_receipt(
    receipt: &alloy::rpc::types::TransactionReceipt,
) -> Result<Vec<[u8; 32]>, EasError> {
    let mut uids = Vec::new();
    for log in receipt.inner.logs() {
        if log.topics().len() >= 2 && log.data().data.len() >= 32 {
            let uid: [u8; 32] = log.data().data[..32]
                .try_into()
                .map_err(|_| EasError::Contract(ContractError::UidExtractionFailed))?;
            uids.push(uid);
        }
    }
    if uids.is_empty() {
        return Err(EasError::Contract(ContractError::UidExtractionFailed));
    }
    Ok(uids)
}
