# Contributing

Keep SLATE aggregate, evidence-labelled, and explicit about the difference
between analysis and pedagogy, assignment, funding, or individual decisions.

Useful public contributions include aggregate source inventories, pathway and
program evidence, workforce or affordability review, privacy review, and safer
public language. For aggregate adaptations, start with
[`docs/adoption/README.md`](docs/adoption/README.md).

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p slate-cli -- --help
```

Do not commit restricted datasets, credentials, local build state, student
records, personal records, or uncited public claims.
