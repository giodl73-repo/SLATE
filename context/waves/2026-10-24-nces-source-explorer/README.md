# Frozen NCES source explorer

Accepted WP-WEB-002 precedes implementation. The first bounded public source
slice contains NCES CCD Table 2, 2023–24 and 2024–25 Final Version 1a: 59 rows per
year, including 51 state/DC rows, US total and seven separate territory/agency
rows. Original HTML, source notes, custody hashes and deterministic normalized
data accompany the browser task. Public aggregate sources only; no person data.

Reader task: select a published jurisdiction, compare matched counts/ratios,
inspect missingness, coverage and reconciliation, and download the exact Rust
result. Native source/query/derivation rules run in the existing WASM worker.
Tennessee's missing staff holds the national teacher total/ratio; Guam's source
quality suppression remains visible. No partial sum, zero or inferred ratio fills
a missing observation. Class size, staffed seats, access and outcomes stay null.
The original fictional graph is separate and has no source-to-graph calibration.

Exit: 41 native tests; format/check/strict Clippy and target Clippy; source hashes
and 118 re-extracted rows verified; 14 executing-WASM browser cases including
four native/source canonical comparisons and both original-source SHA-256 checks.
Desktop/mobile source layouts inspected. Approximately 637 KB artifact. No new
runtime dependency or original network/scoring-core change. Work-package notes
record the implementing-agent role review, marker correction and named defers.
Hosted native/browser checks gate master publication; TRACKER owns deployed byte
identity, source-task and TAXLANE education roundtrip receipts before intake.
