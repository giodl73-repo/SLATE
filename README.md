# SLATE

**Education Access 2.0 — multi-scale education-delivery network analysis.**

**A seat is not access if the student cannot reach, enter, afford, or progress through it.**

SLATE scores schools, institutions, educators, programs, attendance boundaries,
feeder paths, transfers, and articulation pathways across access, capacity,
workforce, program breadth, affordability, continuity, equity, and resilience.

**Series:** [Applied Systems](https://github.com/giodl73-repo/giodl73-repo/blob/main/series/applied-systems.md)

> SLATE is not a pedagogical study, accreditation/licensing determination,
> funding or assignment decision, student-level advice, or advocacy brief, and
> it claims no district, state, ministry, board, institution, or funder endorsement.

## Why this matters

Education capacity is non-fungible: an available seat in the wrong location,
grade, language, program, pathway, or support environment does not close the
gap. SLATE tests whether SHIELD's service-network transfer strains recur in
education rather than forcing a physical-flow model onto students and schools.

## What is implemented

| Crate | Responsibility |
|---|---|
| `slate-network` | Education elements and pathway contracts. |
| `slate-corpus` | Evidence-labelled corpus validation. |
| `slate-score` | DIM-01..13 score artifacts. |
| `slate-tier` | Tier-SLA classification and shortfalls. |
| `slate-gap` | Gap analysis, transfer-strain evidence, and null results. |
| `slate-cli` | Corpus, score, tier-SLA, and gap commands. |

The implementation baseline is complete and fixture-backed. No student records
or individual educational recommendations belong in this repository.

## Quick start

```powershell
cargo run -p slate-cli -- --help
cargo test --workspace
```

## Method

```text
CORPUS -> SCORE -> TIER-SLA -> GAP -> CONCEPT -> REVIEW -> DESIGN
```

## Documentation

- [`PRODUCT_PLAN.md`](PRODUCT_PLAN.md)
- [`docs/vtrace/`](docs/vtrace)
- [`context/waves/`](context/waves)
- [`.roles/ROLE.md`](.roles/ROLE.md)

## License

MIT. See [`LICENSE`](LICENSE).
