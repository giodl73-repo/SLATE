# SLATE — House Rules

## 1. Project Identity

SLATE is a **research and conceptual-design project for Education Access 2.0** — a data-driven upgrade plan for the education delivery network (schools and institutions, elementary, middle, secondary, postsecondary, magnet/CTE/specialized programs, educators, and feeder/attendance-boundary/transfer/articulation pathways), applicable at international, national, regional, and local scales. The mission: score an existing network against a calibrated dimension pool, find the gaps (travel-time and seat access, class-section and teacher capacity, workforce shortage, feeder/articulation discontinuity, program-breadth gaps, affordability barriers, enrollment-surge fragility, tier-SLA shortfalls), and design into them.

**The architectural bet** — borrowed from ROUTE/PYLON/GAUGE/BASIN/PACKET/TARMAC/HARBOR/DRAIN/SHIELD: score enough of an existing network on enough dimensions and the design space tells you its own structure. SLATE is the second deliberate service-and-human stress test and a direct corroboration test for SHIELD's transfer-strain findings. The gaps aren't invented; they're found. A project designed into a real gap is better evidence than one invented from first principles.

**The testable hypothesis**: there is a set of ≤20 interventions — at a stated scale — that, if built or adopted to Education 2.0 standards, would shorten time-to-access, close seat/teacher-shortage and program gaps, strengthen feeder/articulation continuity, and harden enrollment-surge resilience. **A rigorous null result is as valid as a positive one.** A dimension that does not transfer cleanly from physical lifelines, or from SHIELD's healthcare stress test, is reported as a finding, not forced into the model.

Sibling projects: **ROUTE** (highways), **PYLON** (grid), **GAUGE** (rail), **BASIN** (water), **PACKET** (internet), **TARMAC** (air), **HARBOR** (ports), **DRAIN** (wastewater), and **SHIELD** (healthcare access). SLATE borrows their structural patterns; SLATE's own rules apply here.

## 2. Multi-Scale Rule

Every corpus element carries a **scale** (`international` / `national` / `regional` / `local`) and a market/jurisdiction. Scores, tiers, gaps, and design proposals are interpreted **within their stated scale**. A claim must not compare or aggregate across scales without saying so. The same dimension pool and tier model apply at every scale; only the scope of the run changes.

## 3. The Pipeline

```
CORPUS (score existing networks) → RUBRIC CALIBRATES → GAP MAP
  → CONCEPT → SCORE → PARLIAMENT → DESIGN → HANDOFF
```

**Anchor rule**: one existing element must go through the full pipeline (corpus entry → calibration pass → gap-map entry) before any proposed project is analyzed. One proposed project must survive parliament manually before any skill is built. YAGNI is the law.

## 4. Quality Bar

- Research-paper-level estimates. Order-of-magnitude seats, student–teacher ratios, teacher supply, class sections, travel time, feeder/transfer/articulation continuity, affordability, quality/outcome, and cost figures with citations.
- Every number cited. An uncited number blocks promotion to `validated`.
- No capacity or adequacy claim dressed as solved planning — conceptual analysis only, with evidence labels and the demand basis named (`Surge` vs `Baseline`).
- No hand-waving on economics. Marginal or negative benefit-cost projects, funding-formula constraints, boundary constraints, governance fragmentation, and labor constraints are reported as such.
- Data sources declared. Every corpus entry names its source (`data/sources.md`).

## 5. Forbidden Vocabulary

In corpus entries and design proposals: no "obviously needed," "critical gap," "long overdue," or any pre-judged framing before the score supports it. Claims must cite (a) dimension, (b) score, (c) corpus comparison, (d) scale. "This district scores 8.4 on Access vs. a corpus mean of 5.1 at regional scale" beats "this is a critical shortage."

## 6. VTRACE Governance

SLATE's planning baseline lives in `docs/vtrace/` and is authored one deliverable at a time to a `.roles` review fixed point. The initial implementation baseline now exists, but future code, corpus, or public-claim changes still need work-package scope, validation, and review.

## 7. Review Panel

Seven adversarial parliament voices and a three-role editorial gate review every promotable artifact. See `.roles/ROLE.md`. No voice is skipped. The funding-and-governance realist exists because per-pupil funding formulas, property-tax-base disparity, district boundary and governance fragmentation, charter/choice/voucher dynamics, board governance, and teacher-labor constraints govern what schooling actually gets built, kept open, or closed — that market and governance tension is a feature, not an accident.

## 8. Portfolio Discipline

SLATE implementation changes belong in this repo. TRACKER receives only intentional submodule pointer updates after intake. Do not make build or validation correctness depend on TRACKER-relative paths.
