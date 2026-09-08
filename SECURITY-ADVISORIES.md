# Dependency audit triage — 2026-09-08

`cargo audit` on this repo reports 3 vulnerabilities and 3
unmaintained-dependency warnings. Findings are recorded here per the
policy that audit failures are triaged in a dated note, never
soft-skipped.

## Vulnerabilities (blocking cargo audit)

| ID | Crate | Fix | Status |
|---|---|---|---|
| RUSTSEC-2026-0220 | ruint 1.16.0 (shift overflow flags) | >= 1.20.0 | **blocked** — see below |
| RUSTSEC-2025-0137 | ruint 1.16.0 (unsound `reciprocal_mg10`) | >= 1.17.1 | **blocked** — see below |
| RUSTSEC-2026-0009 | time 0.3.44 (DoS via stack exhaustion, medium 6.8) | >= 0.3.47 | transitive via alloy; `cargo update -p time` resolves nothing newer under the current tree |

**Why blocked:** ruint 1.20.0 requires `serde >= 1.0.220`, and this
crate pins `serde < 1.0.217` for alloy 1.0.22 compatibility (rationale
recorded in commit `db98b15` of 2026-04-09). Lifting the serde cap is a
deliberate compatibility decision — it must be verified against the
current alloy release, not done silently to clear an audit. Until that
decision is taken, `cargo update -p ruint --precise 1.20.0` fails
resolution.

**Blast radius:** ruint is a transitive dep of alloy-primitives (256-bit
integer math); this crate's use is attestation UID formatting and
on-chain reads, not attacker-facing parsing paths. time is used by
alloy's provider layer. Neither is in this crate's own direct dependency
surface.

**Path to clear:** decide the serde cap (keep vs lift) for this crate's
dependency-pinning step; if lifted, `cargo update` picks up ruint >=
1.20 and re-run `cargo audit`.

## Unmaintained (warnings, not vulnerabilities)

| ID | Crate | Note |
|---|---|---|
| RUSTSEC-2024-0388 | derivative 2.2.0 | transitive of alloy; upstream choice |
| RUSTSEC-2024-0436 | paste 1.0.15 | transitive of alloy; upstream choice |
| RUSTSEC-2026-0173 | proc-macro-error2 2.0.1 | transitive of alloy; upstream choice |

All three clear when alloy updates its own proc-macro dependencies; no
action available in this repo.
