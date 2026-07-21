# Specification Baseline

## Scope

Repo: SLATE

Baseline type: target (provisional)

Baseline date: 2026-06-26

VTRACE adoption scope: define the controlled behavior SLATE intends to build — the dimension pool, scoring scale, demand basis, corpus schema, evidence labels, tier model, and the **multi-scale model** — before architecture, interfaces, or implementation planning. Because SLATE is greenfield, every item is `target`, not observed `current`. The dimension pool is **provisional**: dimensions and their basis are controlled here, but per-dimension anchors and rubric weights calibrate from the scored corpus (REQ-006) and are not asserted in this file. Future work packages must cite `SPEC-*` / `DIM-*` IDs instead of making unanchored changes.

## Specification Sources

| Source | Evidence | Status | Notes |
|---|---|---|---|
| `README.md` | SLATE thesis, hypothesis, multi-scale, pipeline, SHIELD corroboration posture. | target | Public-facing repo intent. |
| `PRODUCT_PLAN.md` | Scope, non-goals, method, waves, service-and-human recurrence test. | target | Product framing. |
| `CLAUDE.md` | House rules, multi-scale rule, pipeline, quality bar. | target | Operating constraints. |
| `docs/vtrace/MISSION.md` | `NEED-*`, `CON-*`. | current | VTRACE mission source. |
| `docs/vtrace/CONOPS.md` | `OPS-*` scenarios. | current | VTRACE scenario source. |
| `docs/vtrace/REQUIREMENTS.md` | `REQ-001..016`, `DEF-001..005`. | current | VTRACE requirement source. |
| `.roles/ROLE.md` | Parliament + editorial review lenses. | current | Review-lane source. |

## Scale Model (`SCALE-*`) (resolves NEED-008 / REQ-016)

SLATE runs the same methodology at any scale. Every corpus element declares a scale; scores, tiers, and gaps are interpreted within scale.

| Scale | Meaning | Example governance |
|---|---|---|
| `international` | Cross-border/global education-system benchmarking. | International education bodies, multilateral benchmarking programs, cross-border postsecondary or credentialing agreements. |
| `national` | A national education system, federal program, or country-wide institution network. | Education ministries, NCES-equivalent agencies, national education systems, federal programs. |
| `regional` | District, county, metro, state, feeder pattern, postsecondary region, or multi-institution education market. | State agencies, districts, county offices, regional consortia, higher-education systems. |
| `local` | A school, campus, program, attendance zone, pathway, or articulation agreement. | School boards, campus leaders, charter authorizers, local districts, institutions. |

| ID | Rule |
|---|---|
| SCALE-01 | Every corpus element carries a `scale` and a `market`/jurisdiction tag (REQ-016). |
| SCALE-02 | Scores, tiers, and gaps are interpreted within the element's scale; cross-scale comparison/aggregation requires an explicit labelled note (CON-007). |
| SCALE-03 | Scale may nest (a local school within a regional feeder pattern within a national or international benchmark); nesting representation is provisional (DEF-005). |

## Dimension Pool (`DIM-*`)

The candidate pool SLATE scores existing education delivery elements against. Each dimension is scored 0–10. Anchors and weights are **calibrated from the corpus** (REQ-006), not fixed here. "Primary basis" names where the input comes from; "Default label" is the evidence posture a fresh value carries until upgraded with a cited source (REQ-002, REQ-003).

| DIM ID | Dimension | What it measures | Primary basis | Default label |
|---|---|---|---|---|
| DIM-01 | Access | Geographic/travel-time access and seat availability. | NCES CCD, state report cards, Census/ACS, GIS travel-time/transportation data | heuristic |
| DIM-02 | Capacity | Seats, student-teacher ratio, class sections, and per-capita capacity against declared demand basis. | NCES CCD/IPEDS, EDFacts, state staffing/capacity inventories | source-needed |
| DIM-03 | Quality/Outcomes | Achievement, graduation, attainment, and outcome indicators appropriate to the element and tier. | State report cards, EDFacts, SEDA, IPEDS outcomes | source-needed |
| DIM-04 | Connectivity/Pathway Continuity | Feeder, transfer, attendance-boundary, articulation, and next-level continuity and redundancy. | Feeder maps, attendance boundaries, transfer/articulation records, program inventories | heuristic |
| DIM-05 | Resilience | Enrollment-surge capacity, redundancy, closure/overcrowding response, and continuity under shock. | Enrollment projections, displacement/migration data, utilization and closure proxies | heuristic |
| DIM-06 | Workforce | Teacher supply, shortage, staffing, pipeline, and educator availability. | NCES/EDFacts teacher data, state workforce boards, ACS/BLS | source-needed |
| DIM-07 | Affordability/Cost Barriers | Fees, transportation cost, postsecondary tuition, coverage, and other cost barriers. | Census/ACS, state finance records, IPEDS tuition, transportation costs | heuristic |
| DIM-08 | Program Breadth | Course/program/specialty availability appropriate to the tier, including AP, CTE, special education, and multilingual services. | CCD/IPEDS program flags, AP/CTE/special education/multilingual availability, state report cards | source-needed |
| DIM-09 | Facility/Asset Condition | Facility age, condition, closure risk, maintenance backlog, equipment posture. | Facility reports, state capital inventories, public filings | proxy |
| DIM-10 | Outcomes/Mobility Impact | Population attainment, mobility, long-run opportunity, and avoidable outcome impact. | SEDA, state outcomes, Census/ACS attainment and mobility data | heuristic |
| DIM-11 | Equity & Disparities | Segregation, opportunity gaps, underserved access, language/disability access, trust, and distributional burden. | CRDC, Census/ACS, SEDA, local equity data | implemented |
| DIM-12 | Benefit-Cost | Benefit per unit cost, population value, affordability, and fiscal constraint. | Public studies, state finance and education cost data, education economics models | heuristic |
| DIM-13 | Tier-SLA Conformance | Degree the element meets its tier's access-time, capacity, program-breadth, and outcomes SLA (derived; shortfall = tier-SLA gap). | Tier model + DIM-01/02/03/08 | heuristic |

Calibration note (per REQ-006, OPS-002): after the first corpus pass, low-variance, redundant, or non-transferring dimensions are retired and informative ones promoted; the rubric version records each change. The pool above is the v0 candidate set, not a final rubric.

## Demand Basis (resolves DEF-002 minimum)

| ID | Rule |
|---|---|
| SPEC-SG-01 | Surge capacity/adequacy claims use `DemandBasis::Surge` and identify the surge basis (demographic bulge, growth, displacement/migration, school closure, disaster relocation, mid-year arrivals, or other enrollment-surge scenario) when known. |
| SPEC-SG-02 | Surge proxies may be used only with explicit proxy/source-needed labels; they must not be presented as observed staffed surge capacity. |
| SPEC-BL-01 | Baseline capacity/adequacy claims use `DemandBasis::Baseline` and identify the average steady-state enrollment basis when known. |
| SPEC-BL-02 | Baseline averages must not be used to imply surge adequacy; any conversion or comparison requires an explicit labelled note. |

## System Tier Model (`T1–T4`) (resolves NEED-007 / REQ-014 / REQ-015)

SLATE classifies each element into a four-tier hierarchy — from postsecondary/specialized to primary/elementary — with access time, capacity, program breadth, and outcomes SLA terms per tier. This is the Education 2.0 analog of the portfolio tiering. Roles are typical, not strict.

| Tier | Name | Typical role | SLA promise (target) |
|---|---|---|---|
| T1 | Postsecondary / Specialized | University, research institution, CTE/magnet/specialized program, or advanced pathway hub. | Broadest program breadth; specialized capacity; reliable transfer/articulation pathways; outcomes and surge posture appropriate to specialized education. |
| T2 | High School / Secondary | Secondary school or regional high-school program preparing students for postsecondary, workforce, or specialized pathways. | Regional secondary access; adequate staffed seats and sections; program breadth including graduation, CTE/AP/specialized options where promised; defined outcome expectations. |
| T3 | Middle / Intermediate | Middle/intermediate school connecting primary to secondary pathways. | Local intermediate access; adequate seats and teacher capacity; feeder continuity to T2; stable outcome and transition expectations. |
| T4 | Primary / Elementary | Universal front-line elementary access point. | Timely nearby primary access; seat availability; foundational program breadth; continuity into T3; outcomes appropriate to early grades. |

Each tier's SLA is expressed over four contract terms, assessed by DIM-13:

| SLA term | Meaning | Backing dimensions |
|---|---|---|
| Access time | Travel time, distance, enrollment availability, and time-to-service the tier promises. | DIM-01, DIM-11 |
| Capacity | Seats, teachers, sections, and staffed throughput against the declared demand basis. | DIM-02, DIM-05, DIM-06 |
| Program breadth | Course/program/specialty availability appropriate to the tier. | DIM-08, DIM-04 |
| Outcomes | Quality/outcome and mobility performance appropriate to the tier and population. | DIM-03, DIM-10 |

SLA values per tier are **target and provisional** — exact thresholds calibrate with the rubric (REQ-006) and are not asserted here. A tier-SLA shortfall is a first-class gap (REQ-015, OPS-006).

## Controlled Specification Items

| Spec ID | Parent REQ IDs | Type | C/T/D/U | Specification Statement | Verification Method | Validation Method | Owner | Risk | Status |
|---|---|---|---|---|---|---|---|---|---|
| SPEC-001 | REQ-004 / REQ-005 | architecture | target | Every element is keyed by a stable school/institution/program/pathway/boundary/network identifier; operator, board action, vendor, and map id are mutable presentation fields, not keys. | schema check / inspection | OPS-001 | SLATE maintainer | high | accepted |
| SPEC-002 | REQ-001 / REQ-003 / REQ-014 / REQ-016 | product | target | A corpus entry is one markdown file with frontmatter (id, type, scale, market/jurisdiction, school/pathway/boundary, tier, sla, source rows) and a scored dimension block, regenerable from documented commands. | inspection / command review | OPS-001 | SLATE maintainer | medium | accepted |
| SPEC-003 | REQ-002 | product | target | Every quantity carries an evidence label from {implemented, heuristic, simulated, proxy, planned, held, source-needed, confidence-limited}. | artifact inspection | OPS-001 / OPS-004 | SLATE maintainer | medium | accepted |
| SPEC-004 | REQ-006 | product | target | The dimension pool is `DIM-01..DIM-13` scored 0–10; anchors and weights are calibrated from corpus variance and transfer suitability and versioned, not fixed in this baseline. | calibration record / version diff | OPS-002 | SLATE maintainer | high | accepted |
| SPEC-005 | REQ-007 | software | target | Capacity/adequacy dimensions name the demand basis (`Surge` vs `Baseline`) on each claim and follow SPEC-SG-01/02 and SPEC-BL-01/02. | analysis / inspection | OPS-003 | operations reviewer | high | accepted |
| SPEC-006 | REQ-008 | product | target | An already adequate, accessible, staffed, continuous, affordable, equitable, resilient education network — or a non-transferring dimension — is recorded as a labelled null/transfer finding; scope is not expanded to manufacture a gap. | gap-artifact inspection / review | OPS-003 | SLATE maintainer | high | accepted |
| SPEC-007 | REQ-009 / REQ-010 | ops | target | Promotable claims pass the 7-voice parliament and 3-role editorial gate, with access, capacity, quality/outcomes, pathway continuity, resilience, workforce, affordability/cost barriers, program breadth, asset condition, outcomes/mobility impact, equity/disparities, benefit-cost, tier-SLA, funding formulas, property-tax-base disparity, boundary/governance fragmentation, charter/choice/voucher dynamics, board/union governance, and labor constraints represented. | review inspection | OPS-004 | review steward | medium | accepted |
| SPEC-008 | REQ-011 | product | target | Outputs carry a scope boundary: research/tooling/conceptual-design only; no pedagogical/curriculum study, student-level advice, school-accreditation/licensing determination, funding/boundary/charter/voucher/labor/assignment determination, or endorsement. | editorial review | OPS-004 | SLATE maintainer | medium | accepted |
| SPEC-009 | REQ-003 | data | target | `data/sources.md` is the citation registry; every cited quantity names a registry entry, and proxies/heuristics (including NCES CCD, IPEDS, EDFacts, CRDC, Census/ACS, SEDA, state report cards, or modelled values) are labelled rather than silently treated as proof. | citation audit | OPS-001 | data steward | high | accepted |
| SPEC-010 | REQ-012 / REQ-013 | ops | target | VTRACE deliverables advance one at a time to a `.roles` fixed point; SLATE changes stay in the child repo until an intentional TRACKER pointer update after intake. | wave ledger / status review | OPS-005 | SLATE maintainer | low | accepted |
| SPEC-011 | REQ-014 | product | target | Every analyzed element is classified into exactly one tier (T1–T4) per the System Tier Model and carries that tier's declared SLA terms. | schema check / inspection | OPS-006 | SLATE maintainer | high | accepted |
| SPEC-012 | REQ-015 | software | target | Tier-SLA conformance (DIM-13) is assessed per element against its tier SLA; any shortfall is recorded as a tier-SLA gap and a market is not called adequate while an unaddressed shortfall stands. | analysis / gate / inspection | OPS-003 / OPS-006 | SLATE maintainer | high | accepted |
| SPEC-013 | REQ-016 | product | target | Every element carries a `scale` and `market`/jurisdiction tag (SCALE-01); analysis runs within a scale and any cross-scale comparison carries an explicit labelled note (SCALE-02). | schema check / gate / review | OPS-007 | SLATE maintainer | high | accepted |

## Public Contracts

| Contract ID | Spec IDs | Surface | Compatibility Rule | Change-Control Trigger | Verification Evidence |
|---|---|---|---|---|---|
| IF-001 | SPEC-001 / SPEC-002 / SPEC-013 | corpus file (markdown + frontmatter, incl. scale/market) | Frontmatter keys are additive; `id` immutable; `scale` from a fixed enum. | Any key rename/removal, id-semantics, or scale-enum change. | schema check (target) |
| IF-002 | SPEC-009 | `data/sources.md` (registry) | Source entries are append/annotate; ids stable. | Removing or re-pointing a source id. | citation audit (target) |
| IF-003 | SPEC-004 | rubric version record | Dimension set + weights versioned; changes recorded. | Retiring/adding a `DIM-*` or changing weights. | calibration record (target) |
| IF-004 | SPEC-011 / SPEC-012 | `tiers.toml` tier/SLA record | Tier set (T1–T4) and per-tier SLA terms are versioned; tier reassignment is recorded. | Changing a tier definition, SLA term, or an element's tier. | tier/SLA record (target) |

## Package / Language Allocation

| Spec IDs | Package / Module | Responsibility | Forbidden Responsibility | Validation Profile |
|---|---|---|---|---|
| SPEC-001 / SPEC-005 | education delivery graph kernel (future `slate-network`) | Graph model, identity, connectivity, incident capacity, diverse paths, typed demand basis. | Scoring policy, evidence labels, review logic. | L1 |
| SPEC-002 / SPEC-003 / SPEC-009 / SPEC-013 | corpus + data layer | File schema, scale/market tags, source registry, evidence labels. | Graph math, design proposals. | L0/L1 |
| SPEC-004 | scoring layer | DIM-01..13 scoring and versioned rubric. | Tier SLA decisions without tier layer. | L1 |
| SPEC-007 / SPEC-008 | review layer (`.roles`) | Parliament/editorial gate, scope boundary, transfer findings. | Computing scores. | L0 |
| SPEC-011 / SPEC-012 | tier/SLA layer | Tier classification, SLA terms, tier-SLA conformance (DIM-13). | Setting calibrated SLA thresholds without rubric. | L1 |

## Nonfunctional Constraints

| Constraint ID | Parent Spec IDs | Constraint | Threshold / Rule | Verification Method | Status |
|---|---|---|---|---|---|
| SPEC-NF-001 | SPEC-002 / SPEC-004 | Reproducibility | Active corpus/score/tier/gap artifacts regenerate from documented commands with labels and scale preserved. | command review | proposed |
| SPEC-NF-002 | SPEC-009 | No raw datasets committed | Raw/cache data is gitignored; only derived, cited artifacts are committed. | inspection | proposed |
| SPEC-NF-003 | SPEC-001 / SPEC-013 | Deterministic identity + scale | Element ids and scale tags are deterministic given source inputs. | inspection / test | proposed |

## Assumptions And Unknowns

| ID | Item | Impact | Disposition | Owner |
|---|---|---|---|---|
| SPEC-UNK-001 | Cross-scale availability for DIM-01/02/03/06/07 varies by country, state, district, institution, and public-reporting regime. | May force proxy/source-needed labels on early corpus rows and limit comparisons. | discovery → `data/sources.md` | data steward |
| SPEC-UNK-002 | Whether `has_diverse_path` / edge-based connectivity is meaningful when edges are attendance-boundary, feeder, transfer, or articulation relationships rather than physical conduits carrying conserved flow. This recurs from SHIELD SPEC-UNK-002 for referral/catchment edges. | DIM-04 and resilience metrics may need redefinition; physical-network connectivity may partially fail to transfer. | accept risk → calibration finding | education-system planner |
| SPEC-UNK-003 | Whether a single 0–10 score can fairly combine physically measurable access/capacity with socially constructed equity, segregation, affordability, and outcomes dimensions. This recurs from SHIELD SPEC-UNK-003 for measurable + constructed dimensions. | Rubric may need non-additive reporting or dimension-family outputs instead of a single combined score. | defer to corpus calibration | equity/outcomes/mobility reviewers |
| SPEC-UNK-004 | Whether seat/teacher capacity is fungible enough for a simple capacity dimension; an AP-calculus seat, special-education seat, kindergarten seat, multilingual support slot, and CTE section are not interchangeable. This recurs from SHIELD SPEC-UNK-004 for non-fungible beds/providers/service lines. | DIM-02 and tier-SLA thresholds may overstate adequacy unless program, grade, staffing, and support constraints are explicit. | accept risk (labelled basis) | operations reviewer |
| SPEC-UNK-005 | Whether scale nests as a hierarchy or stays a flat tag. | Affects schema + cross-scale notes. | defer (DEF-005) | SLATE maintainer |

## Requirement-To-Spec Coverage

| Requirement ID | Spec IDs | Coverage Status | Notes |
|---|---|---|---|
| REQ-001 | SPEC-002, SPEC-NF-001 | covered | Regeneration path. |
| REQ-002 | SPEC-003 | covered | Evidence labels. |
| REQ-003 | SPEC-009 | covered | Citation registry. |
| REQ-004 | SPEC-001 | covered | Stable identity. |
| REQ-005 | SPEC-001, SPEC-013 | covered | Hold/reject unidentified/untagged rows. |
| REQ-006 | SPEC-004, IF-003 | covered | Calibrated rubric and transfer suitability. |
| REQ-007 | SPEC-005, SPEC-SG-01/02, SPEC-BL-01/02 | covered | Demand basis named. |
| REQ-008 | SPEC-006 | covered | Null/transfer result. |
| REQ-009 | SPEC-007 | covered | Review gate. |
| REQ-010 | SPEC-007 | covered | Stakeholder lenses incl. funding/governance. |
| REQ-011 | SPEC-008 | covered | Scope boundary. |
| REQ-012 | SPEC-010 | covered | Child-repo scoping. |
| REQ-013 | SPEC-010 | covered | One-at-a-time VTRACE. |
| REQ-014 | SPEC-011, IF-004 | covered | Tier classification + SLA. |
| REQ-015 | SPEC-012, DIM-13 | covered | Tier-SLA gap gating. |
| REQ-016 | SPEC-013, SCALE-01..03, IF-001 | covered | Multi-scale tagging + within-scale interpretation. |

## Spec-To-Verification Coverage

| Spec ID | Verification IDs / Commands | Expected Result | Evidence Pointer | Status |
|---|---|---|---|---|
| SPEC-001..013 | future `VER-*` in `VERIFICATION.md` | Each spec has a credible check (schema, command, inspection, or review). | future `EVID-*` | planned |

## Role Review Notes

| Role Lens | Spec Impact | Disposition |
|---|---|---|
| Scope Keeper | Baseline defines controlled behavior, a candidate pool, a tier model, and the scale model; it asserts no scored network or design. | pass |
| Citation Auditor | No quantities asserted; primary bases name where inputs come from; DIM default labels enforce citation discipline. | pass |
| Numeracy Checker | Units are listed but no computed values are asserted; the system `scale` enum is distinct from the score scale. | pass |
| Operations & Enrollment Officer | Demand basis is controlled (`Surge`/`Baseline`); non-fungible seat/teacher/program capacity and surge-vs-baseline ambiguity are named unknowns. | pass_with_risk |
| Funding & Governance Realist | Initial draft made education capacity feel unconstrained; resolved by adding funding-formula/boundary/governance/labor constraints to SPEC-007 and REQ-010. | resolved |
| Education-System Planner | Pathway-continuity connectivity is controlled but explicitly marked as a transfer-strain unknown that recurs from SHIELD SPEC-UNK-002. | pass_with_risk |
| Outcomes/Mobility, Equity & Educator advocates | Equity/disparities (DIM-11), outcomes/mobility impact (DIM-10), workforce (DIM-06), and program breadth (DIM-08) are in the pool. | pass |

Fixed-point note: one actionable finding (funding/governance constraints underplayed) was raised and applied. No unresolved critical or major finding remains. Pool, SLA, scale-nesting, pathway-connectivity semantics, single-score fairness, and capacity fungibility are explicitly provisional; calibration and DEF-005 deferred. SPEC-UNK-002/003/004 explicitly record recurrence from SHIELD.

## Specification Gate

Decision: pass_with_risk

Required before implementation planning:
- [x] Every accepted `REQ-*` maps to one or more `SPEC-*` IDs or a recorded deferral.
- [x] Every implementation work package can name parent `SPEC-*` IDs or discovery status.
- [x] Public contracts have owners and change-control triggers.
- [~] Unknowns are resolved, blocked, deferred, or converted to discovery work (SPEC-UNK-001..005 are discovery/defer/accept-risk).
- [x] Verification and validation methods are credible for the controlled claim.
Rationale: the baseline is coherent enough to drive trace, verification, and the review gate. Residual risk is concentrated in cross-scale data openness, pathway-edge transfer semantics, single-score fairness, non-fungible seat/teacher/program capacity, provisional weights/SLA thresholds, and scale-nesting representation, all deferred to the corpus calibration wave rather than blocking the minimum slice.

