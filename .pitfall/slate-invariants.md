# SLATE Invariants

## SLATE-I-01: Scale, Source, And Demand Basis Are Non-Optional

**Status:** VERIFIED

**Claim:** Promoted corpus evidence preserves stable identity, scale, source
labels, and `DemandBasis`.

**Why it matters:** Education access and capacity claims can be invalidated by
scale, source, or baseline/surge mismatch.

**Enforcement:** Corpus, network, and gap tests cover missing scale, evidence
label preservation, demand basis preservation, and cross-scale filtering.

**Evidence:** `crates/slate-corpus/src/lib.rs`, `crates/slate-network/src/lib.rs`,
`crates/slate-gap/src/lib.rs`, and `cargo test --workspace --locked`.

## SLATE-I-02: Student-Level Data Stays Out

**Status:** VERIFIED

**Claim:** SLATE's current implementation uses synthetic fixtures and aggregate
posture; no student records or individual recommendations belong in the repo.

**Why it matters:** A useful access model can cross a privacy and authority line
if it becomes student-level advice.

**Enforcement:** README/product boundaries, adoption docs, and no-authority
review keep public data use aggregate-only until explicit source review.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `docs/adoption/README.md`, and
`docs/vtrace/REVIEW.md`.

## SLATE-I-03: Null, Tail, Systemic, And Transfer Findings Stay Separate

**Status:** VERIFIED

**Claim:** Gap analysis preserves null results, tail/systemic classifications,
cross-scale exclusions, and transfer-risk posture as separate artifacts.

**Why it matters:** A physical-infrastructure gap idiom can manufacture or hide
education service-network risk.

**Enforcement:** `slate-gap` tests cover adequate null results, minority tail,
systemic share, and cross-scale exclusion.

**Evidence:** `crates/slate-gap/src/lib.rs`, `docs/vtrace/REVIEW.md`, and
`cargo test --workspace --locked`.

## SLATE-I-04: Fixture Validation Is Not Public Corpus Validation

**Status:** PARTIAL

**Claim:** Passing synthetic/fixture tests does not authorize a public
education-system finding.

**Why it matters:** The implementation is real, but source-backed public corpus,
privacy, governance, and role review remain future work.

**Enforcement:** VTRACE review and verification keep public corpus promotion
separate from workspace validation.

**Evidence:** `docs/vtrace/VERIFICATION.md`, `docs/vtrace/REVIEW.md`,
`PRODUCT_PLAN.md`, and `context/waves/2026-06-26-slate-implementation/WAVE.md`.

## SLATE-I-05: Public Reuse Requires Boundary Review

**Status:** PARTIAL

**Claim:** Public or downstream reuse requires scope boundary language and role
review before SLATE output is treated as a decision artifact.

**Why it matters:** Aggregate access research can be mistaken for assignment,
funding, accreditation, labor, or governance authority.

**Enforcement:** README/product boundaries, VTRACE review, role panel, and
PITFALL tracking keep public reuse bounded.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `.roles/ROLE.md`,
`docs/vtrace/REVIEW.md`, and `.pitfall/slate-pitfalls.md`.
