# SLATE Product Plan

## Thesis

Score education-delivery networks at a declared scale, identify measurable
access, capacity, workforce, pathway, program, affordability, and resilience
gaps, and test whether SHIELD's service-network transfer strains recur.

## Implemented product shape

- Six-crate Rust workspace for network, corpus, score, tier, gap, and CLI.
- DIM-01..13 scale-aware evidence contracts.
- Explicit transfer-strain and null-result posture.
- Deterministic synthetic fixtures; no student records.

## Next public work

1. Select a bounded public aggregate-data corpus.
2. Publish source, privacy, governance, and interpretation constraints.
3. Separate measurable access/capacity from equity, trust, and outcomes.
4. Review the first finding through education, labor, governance, and equity roles.

## Non-goals

No student-level advice, accreditation, licensing, assignment, boundary,
funding, charter, voucher, labor-contract, or curriculum determination.

## Validation

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p slate-cli -- --help
```
