# Interfaces

## Scope

Repo: SLATE

Interface type: implemented target. Controls SLATE's external and cross-layer boundaries so future
work packages cannot change them silently. IF-001..004 restate the `SPECIFICATION_BASELINE.md`
public contracts; IF-005..006 add the crate API and CLI introduced in `ARCHITECTURE.md`. Current
evidence covers fixture-backed crate and CLI behavior; public aggregate corpus compatibility remains
future validation.

## Interface Inventory

| ID | Interface | Type | Owner | Consumers | Compatibility Rule | Verification |
|---|---|---|---|---|---|---|
| IF-001 | Corpus entry file (incl. scale/market) | file (markdown + frontmatter) | PKG-002 | analysts, PKG-003/004/005, reviewers | Frontmatter keys additive; `id` immutable; `scale` from a fixed enum | VER-004/016 fixture schema |
| IF-002 | fixture source registry | file (registry) | PKG-002 / data steward | citation audit, all scored quantities | Source entries append/annotate; ids stable | VER-003 fixture citation audit |
| IF-003 | Rubric version record | file | PKG-003 | scoring, calibration | Dimension set + weights versioned | VER-006 fixture calibration record |
| IF-004 | `tiers.toml` SLA record | file | PKG-004 | tier classification, gap analysis | Tier set + SLA terms versioned; reassignment recorded | VER-014 schema check |
| IF-005 | `slate-network` library API | API (Rust crate) | PKG-001 | PKG-002..006 | Public types/functions semver; breaking change is change-control | VER-007 / cargo test |
| IF-006 | `slate` CLI | CLI | PKG-006 | maintainers, agents, analysts | Subcommands/flags additive (incl. `--scale`); output schemas versioned | VER-001 / CLI help + tests |

## Interface Details

### IF-001: Corpus entry file

Purpose: one education delivery element as a reviewable, scored, labelled, **scale-tagged** record.

Inputs: frontmatter (`id`, `type`, `scale`, `market`, `school_pathway_or_attendance-boundary`, `tier`, `sla`, source rows) + a dimension-score block (DIM-01..13).

Outputs: a `validated`/`held`/`draft` corpus artifact joinable by `id`, filterable by `scale`.

Errors: missing `id`, uncited quantity, or missing `scale` → held (REQ-005); type/scope/scale drift → Scope Keeper finding.

Versioning: frontmatter keys additive; `id` semantics immutable; `scale` from a fixed enum (international/national/regional/local); schema in `corpus/SCHEMA.md` (deferred).

Evidence: VER-004/016 over fixtures; public corpus validation remains `SLATE-PF-05`.

### IF-002: `data/sources.md` registry

Purpose: the single citation registry; every cited quantity resolves here.

Inputs: source entries (id, publisher, dataset, access, cadence).

Outputs: stable source ids referenced by corpus quantities.

Errors: cited quantity with no registry entry → Citation Auditor finding; NCES/EDFacts/CRDC/Census/SEDA/state-report-card proxy used as observed → must be labelled proxy.

Versioning: append/annotate only; ids stable. Evidence: VER-003 over fixtures; public source registry remains pending.

### IF-003: Rubric version record

Purpose: control the dimension pool and weights as calibration evolves.

Inputs: dimension set (DIM-01..13), weights, calibration rationale.

Outputs: a versioned rubric the corpus scores against.

Errors: retiring/adding a `DIM-*` or changing weights without a version bump → change-control violation. Versioning: explicit version + rationale (REQ-006). Evidence: VER-006 over fixtures; public calibration remains pending.

### IF-004: `tiers.toml` SLA record

Purpose: control the T1–T4 tier definitions and per-tier SLA terms.

Inputs: tier definitions, SLA terms (access time, capacity, program breadth, outcomes), element tier assignments.

Outputs: a versioned tier/SLA contract used by classification and gap analysis.

Errors: changing a tier/SLA term or an element's tier without a record → change-control violation. Versioning: tier set + SLA terms versioned. Evidence: VER-014/015.

### IF-005: `slate-network` library API

Purpose: the graph primitive every layer builds on.

Inputs: `School` nodes and `Pathway` edges with attributes and identity, including `capacity_seats` and typed `DemandBasis` (`Surge` or `Baseline`).

Outputs: graph queries — `add_school`, `add_pathway`, `school_count`, `pathway_count`, `degree`, `is_connected`, `has_diverse_path` (redundant pathway/access pathway), and `incident_capacity_seats`.

Errors: typed errors for unknown/duplicate ids and bad input (no panics on expected bad input).

Versioning: semver; breaking public API change is change-control. Evidence: VER-007.

### IF-006: `slate` CLI

Purpose: orchestrate the pipeline and emit artifacts reproducibly, at a chosen scale.

Inputs: corpus/data paths, subcommands (`corpus`, `score`, `tier-sla`, `gap`), and a `--scale` filter.

Outputs: regenerated corpus/score/tier/gap artifacts with labels, demand basis, and scale preserved.

Errors: non-zero exit + message on missing inputs, missing scale, or gate failure.

Versioning: subcommands/flags additive; output schemas versioned. Evidence: VER-001.

## Open Questions

| ID | Question | Disposition |
|---|---|---|
| IFQ-001 | Exact public corpus frontmatter schema and `corpus/SCHEMA.md` (incl. scale enum). | Defer to first public corpus wave. |
| IFQ-002 | Public CLI output formats (JSON/CSV/markdown) per subcommand. | Defer to public artifact wave. |
| IFQ-003 | Whether FLETCH owns acquisition behind IF-006 or a separate adapter. | Defer to intake. |
| IFQ-004 | Whether `--scale` accepts a nested path (international/national/regional/local). | Defer to DEF-005. |
| IFQ-005 | Whether `has_diverse_path` remains valid for pathway/attendance-boundary edges. | Defer to calibration and FIND-003. |

## Role Review Notes

| Role Lens | Interface Impact | Disposition |
|---|---|---|
| Scope Keeper | IF-001..006 restate or extend controlled contracts; scale is a fixed enum; no scoring/design asserted. | pass |
| Configuration/change-control lens | Every interface has a compatibility rule and change-control trigger. | pass |
| Citation Auditor | IF-002 hardens the citation boundary; proxy-as-observed rule explicit; public source registry still pending. | pass_with_risk |
| Educator / Instruction Lead | IF-005 exposes school/pathway connectivity and capacity as typed queries with error handling, not panics; transfer semantics are calibration risk. | pass |

Fixed-point note: the 2026-08-24 PITFALL pass closed interface documentation drift against the
implemented baseline while keeping public-corpus schema, output-format, and scale-nesting details
deferred to IFQ-001..005. No unresolved critical/major finding.
