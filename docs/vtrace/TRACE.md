# Trace Matrix

## Scope

Repo: SLATE

VTRACE adoption scope: connect SLATE's accepted requirements to mission needs, CONOPS scenarios,
controlled specification items, implemented design surfaces, work packages, verification,
validation, and evidence.

SLATE now has a six-crate Rust baseline over fixture data. The trace therefore separates
implemented fixture-backed behavior from still-deferred public-source corpus validation and
source-backed education claims.

## Requirement Trace

| Req ID | Parent Need / Constraint / Scenario | Specification Item | Design Element (planned) | Work Package | Verification (planned) | Validation | Evidence | Status |
|---|---|---|---|---|---|---|---|---|
| REQ-001 | NEED-001 / CON-003 / OPS-001 | SPEC-002 / SPEC-NF-001 | corpus + data layer | WP-001 done | VER-001 / command review | OPS-001 | EVID-001 | accepted |
| REQ-002 | NEED-002 / NEED-003 / CON-001 / CON-004 / OPS-001 / OPS-004 | SPEC-003 | corpus + data layer | WP-001 done | VER-002 / artifact inspection | OPS-001 / OPS-004 | EVID-002 | accepted |
| REQ-003 | NEED-001 / CON-003 / CON-004 / OPS-001 | SPEC-009 | fixture source registry | WP-001 done / public registry deferred | VER-003 / citation audit | OPS-001 | EVID-003 | accepted |
| REQ-004 | NEED-004 / CON-002 / OPS-001 | SPEC-001 | education delivery kernel (`slate-network`) | WP-002 done | VER-004 / schema check | OPS-001 | EVID-004 | accepted |
| REQ-005 | NEED-004 / NEED-008 / CON-002 / CON-004 / CON-007 / OPS-001 | SPEC-001 / SPEC-013 / SPEC-NF-003 | kernel + corpus layer | WP-001/WP-002 done | VER-005 / gate / data inspection | OPS-001 | EVID-005 | accepted |
| REQ-006 | NEED-002 / NEED-005 / OPS-002 | SPEC-004 / IF-003 | scoring layer | WP-003 done | VER-006 / calibration record | OPS-002 | EVID-006 | accepted |
| REQ-007 | NEED-002 / CON-001 / OPS-003 / OPS-006 | SPEC-005 / SPEC-SG-01 / SPEC-BL-01 | scoring + tier (demand basis) | WP-003/WP-004 done | VER-007 / analysis / inspection | OPS-003 | EVID-007 | accepted |
| REQ-008 | NEED-006 / CON-001 / OPS-003 | SPEC-006 | gap layer + transfer-finding record | WP-005 done | VER-008 / gap inspection / review | OPS-003 | EVID-008 | accepted |
| REQ-009 | NEED-005 / CON-001 / OPS-004 | SPEC-007 | review layer (`.roles`) | deferred → IMPL_PLAN | VER-009 / review inspection | OPS-004 | future EVID-009 | accepted |
| REQ-010 | NEED-003 / NEED-005 / OPS-004 | SPEC-007 | review layer (`.roles`) | deferred → IMPL_PLAN | VER-010 / role review | OPS-004 | future EVID-010 | accepted |
| REQ-011 | NEED-003 / CON-006 / OPS-004 | SPEC-008 | public docs + editorial | deferred → IMPL_PLAN | VER-011 / editorial review | OPS-004 | future EVID-011 | accepted |
| REQ-012 | CON-005 / OPS-005 | SPEC-010 | child repo / git workflow | deferred → IMPL_PLAN | VER-012 / status review | OPS-005 | future EVID-012 | accepted |
| REQ-013 | NEED-005 / OPS-005 | SPEC-010 | wave ledger / `.roles` | deferred → IMPL_PLAN | VER-013 / wave ledger / review | OPS-005 | this matrix + pulses | accepted |
| REQ-014 | NEED-007 / CON-002 / OPS-006 | SPEC-011 / IF-004 | tier/SLA layer | WP-004 done | VER-014 / schema check | OPS-006 | EVID-014 | accepted |
| REQ-015 | NEED-007 / NEED-002 / NEED-006 / OPS-003 / OPS-006 | SPEC-012 / DIM-13 | tier/SLA + gap layer + transfer-finding record | WP-004/WP-005 done | VER-015 / gate / gap inspection | OPS-003 / OPS-006 | EVID-015 | accepted |
| REQ-016 | NEED-008 / CON-007 / OPS-007 | SPEC-013 / SCALE-01..03 / IF-001 | corpus scale tags + gap scale filter | WP-001/WP-005 done | VER-016 / schema check / gate | OPS-007 | EVID-016 | accepted |

## Cross-Stage Cost Barriers

| Source Stage | IDs Covered | Downstream Stage | Cost Barriers Status | Notes |
|---|---|---|---|---|
| Mission needs | NEED-001..008 | REQ-001..016 | covered | Every need drives one or more requirements; NEED-007 → REQ-014/015; NEED-008 → REQ-005/016. |
| Mission constraints | CON-001..007 | REQ-001..016 | covered | Constraints attached where they affect evidence, identity, scope, repo ops, or scale. |
| CONOPS scenarios | OPS-001..007 | REQ-001..016 | covered | Every scenario drives at least one accepted requirement. |
| Requirements | REQ-001..016 | SPEC-001..013 / SPEC-NF-001..003 / IF-001..004 | covered | Specification cost barriers accepted in `SPECIFICATION_BASELINE.md`. |
| Specifications | SPEC-001..013 | `VER-*` / `EVID-*` | fixture-covered | Verification IDs defined in `VERIFICATION.md`; evidence covers fixture-backed crate behavior. |
| Requirements / specs | REQ-* / SPEC-* | `WP-*` | initial slice done | WP-001..006 are implemented for the fixture baseline; public-source corpus waves remain future. |

## Unknowns And Deferrals Trace

| ID | Item | Downstream Owner | Status |
|---|---|---|---|
| DEF-001 | Dimension pool definitions + weights. | `SPECIFICATION_BASELINE` (pool defined, weights calibrate) / corpus wave | partially closed (pool DIM-01..13 defined; weights deferred) |
| DEF-002 | Staffed-seat/educator/section capacity, surge-vs-baseline basis, and non-fungible program capacity. | `SPECIFICATION_BASELINE` (SPEC-SG-02 / SPEC-BL-02 / SPEC-UNK-004) | deferred |
| DEF-003 | Data-source acquisition + cadence. | `data/sources.md` / `VERIFICATION` | deferred |
| DEF-004 | Implementation interfaces (CLI, schema, crates). | `ARCHITECTURE` / `INTERFACES` | closed by implementation baseline |
| DEF-005 | Scale as flat tag vs nested hierarchy. | `SPECIFICATION_BASELINE` (SCALE-03) / `INTERFACES` | deferred |
| SPEC-UNK-001 | Cross-scale data availability for DIM-01/02/03/06/07. | data steward | discovery |
| SPEC-UNK-002 | Pathway/attendance-boundary edge semantics for DIM-04 and diverse-path resilience. | education-system planner | accepted transfer risk |
| SPEC-UNK-003 | Single 0-10 score fairness across physical and social dimensions. | equity/outcomes/mobility reviewers | defer to calibration |
| SPEC-UNK-004 | Non-fungible seat/educator/program capacity. | operations reviewer | accept risk (labelled basis) |
| SPEC-UNK-005 | Scale nesting representation. | SLATE maintainer | defer (DEF-005) |

## Honest Gaps (no faked evidence)
- Current `VER-*` evidence is fixture-backed. It proves the implemented contracts are executable;
  it does not prove a public aggregate education corpus or promoted education result.
- Dimension weights, SLA thresholds, scale nesting, pathway-connectivity semantics, single-score fairness, and capacity fungibility are provisional; the trace does not claim calibrated values.
- Non-transferring dimensions are explicitly traceable through REQ-006/008, SPEC-006, SPEC-UNK-002/003/004, REVIEW findings, and `SLATE-PF-03` rather than hidden.

## Role Review Notes

| Role Lens | Trace Impact | Disposition |
|---|---|---|
| Scope Keeper | Trace links IDs only; asserts no score or design; REQ-016 scale and REQ-008 transfer findings traced. | pass |
| Citation Auditor | Fixture quantities remain local examples; public-source evidence is not promoted. | pass_with_risk |
| Numeracy Checker | Fixture arithmetic is covered by tests; calibration remains provisional. | pass_with_risk |
| Requirements-traceability lens | Every accepted REQ maps to a need/scenario and a spec; cost barriers gaps are labelled, not hidden. | pass |
| Operations Officer | REQ-007/REQ-015 trace the demand basis and tier-SLA gating through SPEC-005/SPEC-012. | pass |
| Education-System Planner | NEED-007/008 trace cleanly to REQ-014/015/016 and SPEC-011/012/013; transfer-strain unknowns remain visible. | pass_with_risk |

Fixed-point note: the trace now records implemented fixture evidence while preserving public-corpus
and transfer-strain gaps as `deferred`/`accepted risk` rather than fabricated.

No unresolved critical/major finding.
