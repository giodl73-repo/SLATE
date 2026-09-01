# SLATE Pitfalls

## SLATE-PF-01: Seat Count Becomes Access Adequacy

**Status:** MITIGATED

**Pattern:** Available seats or institutions are treated as proof that students
can access appropriate education.

**Domain:** Corpus entries, DIM-01..13 scoring, tier/SLA claims, adoption docs,
and public summaries.

**Detection difficulty:** Seat counts look concrete while grade, program,
language, support, affordability, travel, pathway, and governance constraints
need richer evidence.

**Structural solution:** Keep dimensions and demand basis separate, and hold
uncited or unscaled corpus evidence.

**Evidence:** `README.md`, `CLAUDE.md`, `docs/vtrace/SPECIFICATION_BASELINE.md`,
and `crates/slate-score/src/lib.rs`.

## SLATE-PF-02: Student-Level Or Authority Claim Leaks From Aggregate Tooling

**Status:** MITIGATED

**Pattern:** Synthetic or aggregate access analysis is reused as student advice,
assignment, funding, accreditation, licensing, boundary, labor, curriculum, or
institutional authority.

**Domain:** README, adoption worksheets, first public corpus, customer reuse,
and downstream portfolio summaries.

**Detection difficulty:** Education access findings can be useful and urgent,
which invites decision use before source/privacy/governance review is complete.

**Structural solution:** Keep no-student-record and no-authority language
visible, and require full role review before a public corpus finding.
`docs/public-claim-boundaries.v1.json` now records aggregate-analysis authority,
blocked student-level and education-authority claims, required reuse fields, and
Scope Keeper ownership in machine-readable form.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `docs/adoption/README.md`,
`docs/vtrace/REVIEW.md`, `.roles/ROLE.md`,
`docs/public-claim-boundaries.v1.json`, and
`crates/slate-cli/tests/pitfall_policy.rs`.

**Test:** `cargo test -p slate-cli --test pitfall_policy --locked` parses the
public-claim boundary and asserts that aggregate tooling cannot claim
student-level advice, student record use, assignment, funding, accreditation,
licensing, attendance-boundary, labor-contract, curriculum, endorsement, or
advocacy authority.

## SLATE-PF-03: Transfer Strain Gets Forced Into Physical-Infrastructure Model

**Status:** MITIGATED

**Pattern:** Non-conserved pathway edges, social dimensions, and non-fungible
capacity are forced into a physical-flow score instead of reported as transfer
strain.

**Domain:** Requirements, scoring, gap analysis, calibration, and portfolio
methodology.

**Detection difficulty:** The Infrastructure 2.0 method is useful, so
domain-specific non-transfer can look like implementation friction instead of a
method finding.

**Structural solution:** Preserve the service-network transfer-strain class in
SPEC/TRACE/REVIEW and calibration risk.

**Evidence:** `docs/vtrace/REVIEW.md`, `docs/vtrace/TRACE.md`, and
`docs/vtrace/SPECIFICATION_BASELINE.md`.

## SLATE-PF-04: Historical Governance Text Lags Implementation

**Status:** MITIGATED

**Pattern:** VTRACE docs, local instructions, and implementation-wave rows
describe SLATE as greenfield or code-free after the six-crate workspace exists.

**Domain:** Agent instructions, VTRACE verification/trace/review, implementation
wave, portfolio readiness, and research packet.

**Detection difficulty:** Historical planning docs were true when written and
remain internally coherent, but are no longer current status.

**Structural solution:** Update current-state docs to distinguish fixture-backed
implementation from still-open public corpus validation.

**Evidence:** `AGENTS.md`, `CLAUDE.md`, `docs/vtrace/VERIFICATION.md`,
`docs/vtrace/CODE_RIGOR.md`, and
`context/waves/2026-06-26-slate-implementation/WAVE.md`.

## SLATE-PF-05: Fixture Validation Becomes Public Corpus Validation

**Status:** MITIGATED

**Pattern:** Passing workspace tests and synthetic fixtures are treated as proof
of a public education-system finding.

**Domain:** CLI output, first public corpus, adoption docs, customer reuse, and
portfolio status.

**Detection difficulty:** The implementation is real and validated, but the
source-backed public data corpus and education-specific release review are
still future work.

**Structural solution:** Keep fixture validation and public corpus validation
separate until source, privacy, governance, scale, demand-basis, and full role
review evidence exist. `docs/public-claim-boundaries.v1.json` now records
fixture-backed validation authority, blocked public-corpus claims, required
public corpus gates, and Citation Auditor ownership in machine-readable form.

**Evidence:** `docs/vtrace/VERIFICATION.md`, `docs/vtrace/REVIEW.md`,
`PRODUCT_PLAN.md`, `cargo test --workspace --locked`,
`docs/public-claim-boundaries.v1.json`, and
`crates/slate-cli/tests/pitfall_policy.rs`.

**Test:** `cargo test -p slate-cli --test pitfall_policy --locked` parses the
public-claim boundary and asserts that fixture-backed validation cannot claim a
validated public education-system finding, complete source-backed public corpus,
public adequacy result, customer-ready corpus validation, or district, state,
ministry, board, or funder readiness.
