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
- Frozen NCES CCD Table 2 source explorer for 2023–24/2024–25: matched published
  counts/ratios, null-preserving changes and explicit state/DC coverage. This is
  source exploration, with no source-to-graph calibration or service promotion.

## Next public work

1. Expand and independently review the first bounded NCES aggregate-source slice.
2. Add matched grade/program/staffing/access evidence under the published source,
   privacy, governance and interpretation constraints; state ratios alone do not
   establish these properties.
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
