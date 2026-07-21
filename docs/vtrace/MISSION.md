# Mission

## Scope

Repo: SLATE

VTRACE adoption scope: establish the mission baseline for SLATE before creating requirements, specification baselines, trace rows, or work packages. This file is the leftmost VTRACE artifact for the repo and anchors later `REQ-*`, `SPEC-*`, `WP-*`, verification, and validation records. SLATE is greenfield and code-free: this mission defines intent ahead of any implementation, and any implementation must be built by implementation automation from accepted work packages and trace back to the needs and constraints below.

## Mission Need

| ID | Need | Success Criteria | Status |
|---|---|---|---|
| NEED-001 | SLATE shall turn public education delivery data (e.g. NCES Common Core of Data (CCD), NCES IPEDS, U.S. Dept of Education EDFacts, Civil Rights Data Collection (CRDC), Census/ACS demographics and educational attainment, Stanford Education Data Archive (SEDA) outcomes, and state report cards) into a reproducible scored corpus of existing education-network elements. | A maintainer can regenerate the active corpus, score, and gap artifacts from documented commands, with source/proxy/heuristic labels preserved. | accepted |
| NEED-002 | SLATE shall identify and explain education delivery gaps — access/travel-time and seat availability, capacity, quality/outcomes, feeder/transfer/articulation continuity, enrollment-surge resilience, workforce shortage, affordability/cost barriers, program breadth, facility condition, outcomes/mobility impact, equity/disparities, benefit-cost, and tier-SLA shortfalls — without overstating the evidence or hiding the demand basis. | Every material claim is tied to a data artifact, command, source label, confidence label, review record, scale, and declared demand basis (`Surge` vs `Baseline`) where capacity or adequacy is asserted. | accepted |
| NEED-003 | SLATE shall convert analysis into defensible conceptual Education 2.0 upgrade options, not pedagogical/curriculum studies, school-accreditation or licensing determinations, funding/boundary/charter-authorization determinations, student-level advice, or advocacy briefs. | Proposed projects and feature packages are labelled implemented, heuristic, simulated, planned, held, or deprecated, with the demand basis (`Surge` vs `Baseline`), funding-formula, boundary, governance, labor, and economic basis labelled before publication. | accepted |
| NEED-004 | SLATE shall keep network identity stable as analysis moves from raw schools/institutions/programs/educator workforces/attendance boundaries/feeder paths/articulation paths to scored networks, gap regions, and design proposals. | Element-bearing artifacts join through a stable school/institution/program/pathway/boundary/network identifier rather than a transient label, operator, board action, vendor, or map id. | accepted |
| NEED-005 | SLATE shall expose education delivery tradeoffs through adversarial review roles instead of hiding them behind a single score. | Parliament and editorial reviews can change claims, labels, next evidence steps, or promotion status. | accepted |
| NEED-006 | SLATE shall report a rigorous null result, or a non-transferring dimension from physical-lifeline or SHIELD methodology, as a valid finding. | When the scored corpus shows an education network is already accessible, staffed, continuous, affordable, equitable, broad, and resilient — or that a dimension does not transfer cleanly — the artifacts say so rather than manufacturing a gap. | accepted |
| NEED-007 | SLATE shall classify each element into a four-tier hierarchy (T1 Postsecondary / Specialized, T2 High School / Secondary, T3 Middle / Intermediate, T4 Primary / Elementary) and define access time, capacity, program breadth, and outcomes SLAs per tier, so that "is education service adequate here?" is answered against an explicit tier promise. | Every analyzed element carries a tier and a declared SLA, and adequacy claims are made against the tier SLA rather than an unstated baseline. | accepted |
| NEED-008 | SLATE shall apply the same methodology at multiple scales — international (cross-border/global education-system benchmarking), national (a national/state system or program), regional (a district, county, or metro), and local (a single school, campus, or program) — with every element tagged by scale and market/jurisdiction, and analysis runnable at a chosen scale. | Every corpus element declares a scale; scores, tiers, and gaps are interpreted within scale; a gap run can target a single scale without cross-scale leakage. | accepted |

## Users

| User | Need | Success Signal |
|---|---|---|
| SLATE maintainer | Know which commands, artifacts, and review gates define the current truthful repo state at a given scale. | A clean validation bundle runs and the resulting artifacts match the documented claims and declared scale. |
| Education access / services analyst | Inspect scored networks, gaps, and evidence labels without reverse-engineering the implementation. | Scores, gap maps, and reports cite their source surfaces, confidence posture, demand basis, and scale. |
| District / state / articulation planner | Understand why a network, tier, pathway, or project is supported, held, or downgraded. | Each claim names the data, scenario, role review, scale, and next evidence step that governs it. |
| Operations / enrollment / utilization reviewer | See how SLATE handles staffed seats, overcrowding, waitlists, mid-year arrivals, transfers, program slots, and enrollment surge conceptually. | Capacity and adequacy claims expose their demand basis (`Surge` vs `Baseline`) and evidence level, not just an aggregate score. |
| Funding / governance stakeholder | See whether funding formulas, boundaries, board authority, charter/choice/voucher dynamics, capital approval, and teacher-labor constraints are represented honestly. | Funding, boundary, governance, labor, and fiscal-sustainability assumptions are explicit and priced, not assumed free. |
| Student / family / equity / workforce reviewer | See access, affordability, segregation, opportunity gaps, program breadth, attainment, mobility, and workforce pipeline before a project is promoted. | Travel-time, seat-availability, affordability, shortage-area, outcome, mobility, and pathway claims point to data or held evidence, not narrative alone. |
| Coding agent | Make scoped changes without drifting claims, artifacts, scale, demand basis, SHIELD-corroboration honesty, or review obligations. | Work packages name parent IDs, affected modules/data/docs, validation commands, and evidence rows before closure. |

## Operating Context

SLATE will be a data corpus, review system, and research/design process for Education Access 2.0, with any implementation built later by implementation automation from accepted VTRACE work packages. It is **multi-scale by design**: the same corpus, dimension pool, and tier model apply to a school, campus, program, attendance zone, feeder pattern, transfer/articulation pathway, district, state or national education program, postsecondary network, or international education-system benchmark, and a run targets a stated scale. Work happens inside a dirty portfolio checkout, so repo-local changes must stay scoped and must not depend on TRACKER-relative paths for build correctness. SLATE is not yet a TRACKER submodule until intake completes.

This mission file does not yet assert any scored result. It creates the VTRACE anchor that later requirements, specifications, and work packages trace back to.

The tiering frame (NEED-007) and the scale frame (NEED-008) extend the portfolio pattern shared with ROUTE, PYLON, GAUGE, BASIN, PACKET, TARMAC, HARBOR, DRAIN, and SHIELD. SLATE is the portfolio's **second service-and-human network** and a deliberate **corroboration test for SHIELD**. SHIELD showed that physical-lifeline methodology strains when edges are referral/catchment relationships rather than conduits carrying conserved physical flow, when a single 0–10 score risks fusing measurable access/capacity with socially constructed equity/trust/outcomes, and when capacity is non-fungible. SLATE tests whether the same findings recur when nodes are schools, campuses, programs, and educators; edges are attendance-boundary, feeder, transfer, and articulation relationships; and capacity is non-fungible seats constrained by staffing, program type, funding formulas, governance, segregation, affordability, and trust. A non-transferring dimension is reported as a finding, not forced into the model.

## Constraints

| ID | Constraint | Rationale | Status |
|---|---|---|---|
| CON-001 | SLATE public claims must stay bounded by implemented commands, generated artifacts, source labels, confidence labels, and review records. | Prevents planned, heuristic, or simulated work from reading as proof-grade evidence. | accepted |
| CON-002 | Element-bearing artifacts must preserve stable school/institution/program/pathway/boundary/network identity; operators, board actions, vendors, and map ids are not primary keys. | Keeps scores, gaps, and proposals tied to stable service and jurisdictional identity. | accepted |
| CON-003 | Generated artifacts must name the source-of-truth data and commands that regenerate them. | Keeps the repo reproducible and prevents hand-edited generated outputs from becoming hidden state. | accepted |
| CON-004 | Source gaps, heuristic rows, simulated evidence, and human/owner review holds must remain visible status, not missing prose. | Keeps evidence debt actionable and traceable. | accepted |
| CON-005 | SLATE implementation changes belong in this repo; TRACKER receives only intentional submodule pointer updates after intake. | Preserves portfolio snapshot discipline. | accepted |
| CON-006 | SLATE must not claim pedagogical-study findings, student-level advice, construction readiness, school-accreditation/licensing validity, funding/boundary/charter/voucher determination, labor-contract determination, or official agency/district/board/institution/funder endorsement. | Keeps the project framed as research, tooling, review, and conceptual design. | accepted |
| CON-007 | Every claim must declare its scale, and scores/tiers/gaps must not be compared or aggregated across scales without an explicit, labelled cross-scale note. | Prevents misleading mixing of local, regional, national, and international evidence (NEED-008). | accepted |

## Non-Goals
- SLATE is not a pedagogical or curriculum study, school-accreditation review, licensing determination, or student-level advice.
- SLATE is not a funding, boundary, charter-authorization, voucher, labor-contract, or assignment determination.
- SLATE is not an advocacy brief for a specific school, district, university, provider, program, technology, or policy.
- SLATE does not predict what districts, states, ministries, agencies, boards, funders, or institutions will build, close, approve, or call.
- SLATE does not treat illustrative maps or heuristic forecasts as proof-grade evidence unless their evidence level says so.

## Success Criteria

| Criterion | Validation Method | Evidence Pointer |
|---|---|---|
| VTRACE mission needs are explicit enough to derive requirements. | Inspect this file before writing `REQUIREMENTS.md`. | future `EVID-*` |
| Mission needs cover corpus reproducibility, evidence posture, design boundaries, identity, review roles, null-result discipline, non-transfer findings, tiered SLAs, multi-scale applicability, named demand basis, and SHIELD corroboration. | Cross-check against `README.md`, `PRODUCT_PLAN.md`, and `CLAUDE.md`. | future `EVID-*` |
| Later VTRACE artifacts can reference stable parent IDs. | `REQ-*` rows should cite `NEED-*` and `CON-*` IDs from this file. | future `TRACE.md` |
| The physical-lifeline and SHIELD transfer-strain test remains honest. | Requirements and review records must explicitly report whether SHIELD's transfer-strain findings recur in education, and must allow a null result or non-transferring dimension to close as a finding instead of forcing a positive gap. | future `REVIEW.md` |

## Role Review Notes

| Role Lens | Mission Impact | Disposition |
|---|---|---|
| Scope Keeper | Mission stays at repo/system intent; it asserts no scores, gap findings, or design proposals, and names the multi-scale rule. | pass |
| Citation Auditor | Mission makes no quantitative claims beyond ID/tier labels and the ≤20 hypothesis; public source families are named as future corpus inputs. | pass |
| Numeracy Checker | Mission contains no arithmetic, capacity-rate, travel-time, outcome-rate, or cost claims. | pass |
| Education-System Planner | Mission names access, feeder/articulation continuity, tiering, multi-scale, resilience, and public-interest intent. | pass |
| Operations & Enrollment Officer | Mission requires demand-basis framing for capacity/adequacy (`Surge` vs `Baseline`) in NEED-002/003 and the operations user lens. | pass |
| Funding & Governance Realist | Initial draft underplayed funding formulas, property-tax-base disparity, district boundary/governance fragmentation, charter/choice/voucher dynamics, board authority, and teacher-labor constraints; resolved by adding the funding/governance user lens, NEED-003 funding/boundary/governance/labor assumptions, and CON-006 determination boundary. | resolved |
| Equity, Outcomes/Mobility & Educator advocates | Mission names shortage areas, affordability, segregation/disparities, workforce, quality/outcomes, program breadth, mobility, and staffed instruction as first-class via users and NEED-002. | pass |

Fixed-point note: one actionable finding (funding-formula/boundary/governance/labor constraints under-represented; access/capacity claims cannot assume seats are free or boundaries freely redrawn) was raised and applied. No unresolved critical or major finding remains. Deferred: dimension pool, scoring rubric, tier SLA thresholds, demand methodology (`Surge` / `Baseline`), funding/governance constraint schema, SHIELD transfer-strain recurrence handling, and the scale-tagging schema to REQUIREMENTS and SPECIFICATION_BASELINE.

## Source Links
- `README.md`
- `PRODUCT_PLAN.md`
- `CLAUDE.md`
- `.roles/ROLE.md`
