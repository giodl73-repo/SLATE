# Pulse 01: WP-001 `slate-network` education delivery kernel

Status: pending. Executes WP-001 (see `docs/vtrace/WORK_PACKAGES.md`).

## Scope

The school/pathway graph kernel — the pipeline primitive every other crate depends on. Implements

the load-bearing identity, connectivity, and typed demand basis (Surge/Baseline) invariants

required by REQ-007.

## Planned changes
- `Cargo.toml` workspace (member `crates/slate-network`).
- `crates/slate-network/Cargo.toml` (deps: `petgraph`, `serde`, `thiserror`).
- `crates/slate-network/src/lib.rs`: `School`, `Pathway` (with typed `DemandBasis` enum), `Network`,
  `NetworkError`; `add_school`/`add_pathway` (identity + validation); `school_count`, `pathway_count`,

  `degree`, `is_connected`, `has_diverse_path`, `incident_capacity_seats`.

## Parent IDs

REQ-004/005/007 · SPEC-001/005 · IF-005 · PKG-001 · CR-001..008.

## Exit criteria
- Workspace compiles; `cargo test -p slate-network` green.
- Tests cover: build network; degree; connectivity vs gap; incident capacity; demand basis
  preserved (Surge/Baseline); `has_diverse_path` true on a ring/mesh and false on a

  single-path chain; duplicate-school, non-positive capacity, unknown-school typed errors.
- No `unwrap`/`panic!` in lib paths except tests; `clippy -D warnings` clean.

## Validation
```powershell
cargo fmt --check

cargo clippy --workspace -- -D warnings

cargo test -p slate-network
```

## VTRACE closeout (on completion)

VER-004/005/007 + EVID-CR-001..003 → passed; TRACE REQ-004/005/007 → implemented; WORK_PACKAGES

WP-001 → done; unblock WP-002.

## Status

Pending — ready for implementation automation to execute.
