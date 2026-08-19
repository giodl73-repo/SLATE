# SLATE

**Education Access 2.0 — multi-scale education-delivery network analysis.**

**A seat is not access if the student cannot reach, enter, afford, or progress through it.**

SLATE scores schools, institutions, educators, programs, attendance boundaries,
feeder paths, transfers, and articulation pathways across access, capacity,
workforce, program breadth, affordability, continuity, equity, and resilience.

**Series:** [Applied Systems](https://github.com/giodl73-repo/giodl73-repo/blob/main/series/applied-systems.md)

## Infrastructure 2.0 family

SLATE is one domain implementation of a shared evidence-first method:

```text
PUBLIC SOURCES → CORPUS → SCORE → SERVICE PROMISE → GAP MAP
                                                     ↓
                                      CONCEPT → REVIEW → DESIGN
```

| Lane | Repositories |
|------|--------------|
| Movement | [ROUTE](https://github.com/giodl73-repo/ROUTE), [GAUGE](https://github.com/giodl73-repo/GAUGE), [TARMAC](https://github.com/giodl73-repo/TARMAC), [HARBOR](https://github.com/giodl73-repo/HARBOR) |
| Lifelines | [PYLON](https://github.com/giodl73-repo/PYLON), [PACKET](https://github.com/giodl73-repo/PACKET), [BASIN](https://github.com/giodl73-repo/BASIN), [DRAIN](https://github.com/giodl73-repo/DRAIN) |
| Public access | [SHIELD](https://github.com/giodl73-repo/SHIELD), [SLATE](https://github.com/giodl73-repo/SLATE) |
| Civic boundaries | [ZONES](https://github.com/giodl73-repo/ZONES) |

The family shares evidence labels, explicit scale and demand bases, T1–T4
service promises where meaningful, adversarial review, and acceptance of a
rigorous null result. Each repository owns its domain semantics and safety
boundary.

> SLATE is not a pedagogical study, accreditation/licensing determination,
> funding or assignment decision, student-level advice, or advocacy brief, and
> it claims no district, state, ministry, board, institution, or funder endorsement.

## Use SLATE

SLATE is public and open to use as a reference model for aggregate,
evidence-gated education-access analysis. To scope a safe transfer test, source
review, or aggregate-only local adaptation, start with
[`docs/adoption/README.md`](docs/adoption/README.md).

### Reuse boundary

SLATE is intentionally a domain-specific reference implementation, not a
shared portfolio library. Infrastructure 2.0 repositories share an
evidence-first method and crate topology, but SLATE's seat, educator, pathway,
and support-service types belong here. Reuse the method by comparison, not
through cross-repository crate dependencies. Extract a shared contract only
when a named downstream adopter needs the same stable type or schema in at
least two domains.

## Why this matters

Education capacity is non-fungible: an available seat in the wrong location,
grade, language, program, pathway, or support environment does not close the
gap. SLATE tests whether SHIELD's service-network transfer strains recur in
education rather than forcing a physical-flow model onto students and schools.

## Why this is harder than physical infrastructure

SLATE cannot treat capacity as a fungible physical flow. A seat, educator,
program, grade band, language pathway, transfer route, affordability condition,
and support environment are not interchangeable.

That makes the evidence boundary stricter:

- use aggregate and synthetic fixtures unless a source is explicitly public and
  safe;
- never introduce student records or individual educational recommendations;
- keep accreditation, assignment, funding, licensing, and pedagogy claims held
  unless a qualified external authority and source path support them;
- treat transfer-strain findings as service-network evidence, not student-level
  advice.

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
- [`docs/adoption/`](docs/adoption)
- [`docs/vtrace/`](docs/vtrace)
- [`context/waves/`](context/waves)
- [`.roles/ROLE.md`](.roles/ROLE.md)

## License

SLATE uses separate licenses for software and content. Source code,
executable scripts, tests, configuration, and ordinary software
documentation are MIT-licensed (copyright Gio Della-Libera). Original
non-software content is licensed CC BY-NC 4.0 (copyright Gio Della-Libera);
commercial use of that content requires separate written permission.
Third-party material remains under its own terms.
See [LICENSE](./LICENSE) for the complete notice.
