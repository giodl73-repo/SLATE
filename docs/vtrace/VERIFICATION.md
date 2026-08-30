# Verification Plan

## Scope

Repo: SLATE

VTRACE adoption scope: define verification methods and command levels for SLATE's requirements.

The initial VTRACE baseline was authored while SLATE was greenfield; the six-crate workspace now
exists and the June 2026 implementation wave has closed. This file records current command evidence
while preserving residual limits: the implementation is fixture-backed, public aggregate source
corpus work remains future, and outputs are not student-level advice, accreditation/licensing,
assignment, funding, boundary, charter/voucher, labor, curriculum, district, ministry, board, or
funder authority.

## Verification Matrix

| Req ID | Method | Command / Inspection | Expected Evidence | Result | Evidence Pointer |
|---|---|---|---|---|---|
| REQ-001 | inspection / demonstration | CLI help and deterministic command tests | a documented regeneration path with labels + scale preserved | passed | EVID-001 |
| REQ-002 | inspection / review | corpus tests for evidence-label preservation | every material quantity carries an evidence label | passed | EVID-002 |
| REQ-003 | citation audit | inspect source-labelled corpus/test paths | every cited quantity resolves to a registry source or is labelled | pass_with_risk | EVID-003 (fixture-backed; public corpus future) |
| REQ-004 | schema check / inspection | corpus/network tests for stable ids | stable school/pathway/network id present; labels are not keys | passed | EVID-004 |
| REQ-005 | gate / data inspection | corpus tests for unidentified, uncited, or missing-scale rows | such rows held, not promoted | passed | EVID-005 |
| REQ-006 | calibration record | score/rubric tests and transfer-risk review | rubric changes are versioned and justified; non-transferring dimensions are recorded | pass_with_risk | EVID-006 (v0, provisional) |
| REQ-007 | analysis / inspection | network tests for `DemandBasis` preservation | surge-vs-baseline and `DemandBasis::Surge` vs `DemandBasis::Baseline` named on each claim | passed | EVID-007 |
| REQ-008 | gap inspection / review | gap tests for null, tail, systemic, and cross-scale behavior | null/transfer result recorded, no manufactured gap | passed | EVID-008 |
| REQ-009 | review inspection | confirm parliament + editorial gate ran on a promoted claim | review records exist with dispositions | pass_with_risk | EVID-009 (panel exists, not yet exercised on a corpus claim) |
| REQ-010 | role review | confirm access/capacity/outcomes/pathway/resilience/workforce/affordability/program breadth/assets/outcomes/mobility/equity/benefit-cost/tier-SLA/funding-governance fragmentation lenses represented | stakeholder lenses present in `.roles/` and applied | pass_with_risk | EVID-010 (`.roles/` panel built) |
| REQ-011 | editorial review | inspect public claims for scope boundary | outputs framed as research/tooling/conceptual design | pass_with_risk | EVID-011 (`README`/`PRODUCT_PLAN`/`MISSION` non-goals) |
| REQ-012 | status inspection | repo-local scope inspection; confirm no TRACKER pointer dependency | SLATE changes stay in the child repo | passed | EVID-012 |
| REQ-013 | wave ledger / review | inspect wave ledger + pulses for one-stage/pulse discipline | each VTRACE stage settled to a fixed point in sequence | passed | EVID-013 |
| REQ-014 | schema check / inspection | tier tests | every element classified T1–T4 with declared SLA | passed | EVID-014 |
| REQ-015 | gate / gap inspection | tier and gap tests | tier-SLA shortfalls reported before adequacy claimed | passed | EVID-015 |
| REQ-016 | schema check / gate | corpus/gap tests for scale filter and cross-scale marker | every element scale-tagged; cross-scale notes explicit | passed | EVID-016 |
| REQ-DOC-001 | doc QA | `proof check .` | markdown QA clean across repo docs | passed | EVID-DOC-001 |

## Commands
```powershell

cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p slate-cli -- --help
git diff --check
```

## Validation Levels

| Level | Purpose | Commands / Evidence | Result |
|---|---|---|---|
| L0 | Fast crate/doc sanity for the active stage. | package tests and `git diff --check` | passed |
| L1 | Full repo confidence before push. | fmt, clippy, workspace tests, CLI help | passed |
| L2 | Readiness proof before a public claim. | source-backed corpus + tier/gap replay + role review | pending public corpus |

## Evidence Ledger

| Evidence ID | Type | Path / Command | Covers | Result |
|---|---|---|---|---|
| EVID-DOC-001 | report | `proof check .` (0 errors) | REQ-DOC-001 | passed |
| EVID-012 | inspection | repo-local scope inspection (no code, no TRACKER dependency) | REQ-012 | passed |
| EVID-013 | review | `context/waves/2026-06-26-vtrace-foundation/` ledger + pulses | REQ-013 | passed |
| EVID-009..011 | review | `.roles/` panel present and applied in stage reviews | REQ-009/010/011 | pass_with_risk |
| EVID-001..008, 014, 015, 016 | command/review | fmt, clippy, tests, CLI help, scope review | REQ-001..008/014/015/016 | passed/pass_with_risk |

## Gaps

| Gap | Impact | Disposition |
|---|---|---|
| Public aggregate corpus does not exist yet. | Current implementation is fixture-backed, not a public education-system claim. | keep public-source corpus and role review open |
| Review gate not yet exercised on a source-backed public corpus claim. | REQ-009/010/011 are process-verified, not outcome-verified for public data. | accept risk until first public corpus entry |
| Transfer-strain unknowns unresolved. | DIM-04/diverse-path semantics, single-score fairness, and capacity fungibility remain calibration risks. | accept/defer per SPEC-UNK-002..004 |

## Role Review Notes

| Role Lens | Verification Impact | Disposition |
|---|---|---|
| V&V lens | Methods are credible and mapped 1:1 to requirements; unrun checks are `pending`, not faked. | pass |
| Citation Auditor | Evidence pointers are real (commands run) or explicitly future. | pass |
| Numeracy Checker | The one quantity (0 errors) is a real command result after validation. | pass |
| Scope Keeper | Verification stays at method/result level; REQ-016 scale and REQ-008 transfer-finding checks named. | pass |

Fixed-point note: no actionable finding required a change. The plan honestly separates verified-now

(process/doc) from pending (implementation and transfer calibration). No unresolved critical/major finding.
