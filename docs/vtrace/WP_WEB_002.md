# WP-WEB-002 — Frozen NCES state-source comparison

Status: accepted for implementation automation, 2026-10-24, under the owner's
instruction to continue with source-backed satellite evidence. This work package
authorizes source exploration and bounded software publication, not an access
finding or canonical source-spine promotion.

## Reader task and source contract

Compare the same named jurisdiction in NCES CCD Table 2 for school years 2023–24
and 2024–25, Final Version 1a. Freeze both official public HTML tables and their
notes, hashes and capture time; derive a reproducible normalized representation.
Retain operating schools/districts, student membership, teacher FTE and published
pupil/teacher ratios at their published precision. Student/staff counts are SEA
aggregates; directory counts have school/LEA grains. Pre-K through grade 13 and
adult-education exclusions remain explicit. No student or educator records.

Keep US (50 states plus DC), state/DC and other jurisdiction/agency rows distinct.
No summation of territories/agencies into US totals, no national-vs-state rank.
Preserve em-dash unavailable and double-dagger withheld markers with source
notes. In 2024–25, Tennessee's absent teacher data holds the national teacher
aggregate/ratio; Guam membership is suppressed for source-quality reasons.
No partial national teacher total or inferred ratio fills the gap. Published
ratio is not class size, subject/grade staffing, staffed seats, program fit,
access, outcomes, equity, affordability or governance. Integer FTE displays are
published table precision, not teacher headcount or exact microdata.

Use Rust to validate source rows, query a fixed jurisdiction, compare available
same-field values across the two releases and report 51-state/DC coverage and
reconciliation residuals. Missing operands yield null changes, never zeros.
Expose residuals rather than forcing agreement between rounded FTE counts.
Existing synthetic graph remains a separate task; no source-to-graph mapping,
rubric, tier, gap, assignment, funding, labor, curriculum or savings authority.

## Implementation and exit

Source capture/registry and deterministic extractor; native source adapter in
slate-web and native CLI replay; worker source-query panel; fixtures/source
custody and notices in Pages; README and wave. Typed closed query <=64 KiB;
unknown fields/jurisdictions fail closed. No new runtime dependencies needed.
Original graph/network/scoring cores unchanged. Source refresh is explicit;
the browser neither fetches a live feed nor edits official counts.

Exit: format/check/strict Clippy/workspace tests; exact re-extraction/hash checks;
native/executing-WASM canonical parity for US, state, Tennessee and Guam; missing
and suppressed outputs/null changes; distinct population scopes and source
reconciliation; errors/stale clearing/reset/download/loading/failure; keyboard,
mobile, existing graph regression and TAXLANE education navigation. Include
source originals, normalized data, notes, provenance, WASM and notices in <5 MB.
Hosted checks before master-only publication; exact deployed artifact and live
reader task before a separate TRACKER pointer update. Rollback reverts this
source panel and restores the preceding artifact.

## Role Review Notes

Implementing-agent lenses, no independent endorsement. Parliament first:
education-system planner passes source denominators with access/continuity held;
educator passes FTE distinct from class size and usable instruction; operations
passes historical school years distinct from Surge capacity; economist passes no
cost/value or causal interpretation; equity advocate passes missingness and
separate jurisdictions, with disparities held; outcomes advocate passes no
learning/mobility inference; funding/governance realist passes no formula,
boundary, board, choice or labor determination. Real service conclusions defer
to PRODUCT_PLAN's matched program/access and interpretation work.

Editorial after Parliament: citation auditor passes frozen official originals,
notes and derivations; scope keeper passes named source populations/years and
no graph/rubric promotion; numeracy checker passes integer FTE precision,
published ratios, null operands and visible reconciliation residuals. Student/
family, educator, rural community, workforce/economy and funder lenses retain
their distinct access, workload, shortage, attainment and cost unknowns, deferred
to that matched evidence work. No critical/major actionable finding remains.
Repeat review at implementation exit; advance no service or fiscal hold.
