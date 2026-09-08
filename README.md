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

## Local validation

Run before every commit:

```bash
cargo build --locked && cargo test   # compile + test suite
cargo audit                          # RustSec advisories against Cargo.lock
```

This repo also ships a pre-commit hook (`.githooks/pre-commit`) that blocks
secrets and internal endpoints from entering the public history. Activate it
after cloning:

```bash
git config core.hooksPath .githooks
```

## License

This repository and all contributions are licensed under the [LGPL 3.0](https://www.gnu.org/licenses/lgpl-3.0.html), unless otherwise specified in subdirectory LICENSE files.
