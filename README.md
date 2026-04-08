# antarez-eas-client

A generic Rust client library for the [Ethereum Attestation Service (EAS)](https://attest.org/).

Provides a typed, async interface for creating and verifying on-chain attestations via EAS smart contracts. Designed to be domain-agnostic — usable by any application that needs to attest data on Ethereum or EAS-supported L2s.

## Status

**Not yet implemented.** This repo reserves the crate name and establishes the project structure.

## Planned Features

- Register and manage EAS schemas
- Create on-chain attestations (single and batch)
- Verify attestation existence and validity
- Query attestations by schema, attester, or recipient
- Support for Ethereum mainnet and L2s (Base, Arbitrum, Optimism)
- Async API built on `ethers-rs` / `alloy`

## Usage

This crate will be consumed as a git dependency:

```toml
[dependencies]
antarez-eas-client = { git = "git@github.com:antarez-tech-solutions/antarez-eas-client.git", branch = "main" }
```

## License

Proprietary — Antarez Tech Solutions.
