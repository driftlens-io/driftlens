# Contributing to DriftLens

First off — thank you. DriftLens is early stage and every contribution matters.

---

## Before you start

For anything beyond typos and small fixes, **open an issue first**.
This avoids wasted effort if the change doesn't align with the roadmap.

---

## Local setup

You will need:

- Rust 1.77+ (`rustup` recommended)
- Docker (for running PostgreSQL in tests)
- Java 17+ (for running fixture projects)

```bash
# Clone the repo
git clone https://github.com/driftlens-io/driftlens
cd driftlens

# Build all crates
cargo build

# Run tests
cargo test

# Run integration tests (requires Docker)
docker compose -f tests/docker-compose.yml up -d
cargo test --features integration
```

---

## Project structure

| Directory | Purpose |
|-----------|---------|
| `crates/core` | Shared types — `EntityModel`, `DriftReport`, `DriftKind` |
| `crates/model-reader` | Reads JPA entities from `.java` source and `.class` bytecode |
| `crates/db-probe` | Connects to the database and introspects the real schema |
| `crates/reporter` | Generates HTML, JSON, and Markdown reports from a `DriftReport` |
| `cli` | The main binary — wires everything together |
| `action` | GitHub Action wrapper |
| `tests/fixtures` | Java projects and SQL schemas used in integration tests |

---

## How to add support for a new database

This is the most impactful contribution you can make.

1. Create a new module under `crates/db-probe/src/`
2. Implement the `DbProbe` trait defined in `crates/db-probe/src/probe.rs`
3. Add fixture SQL schemas under `tests/fixtures/sql-schemas/<your-db>/`
4. Add integration tests under `tests/integration/`
5. Document quirks and requirements under `docs/databases/<your-db>.md`

Open an issue tagged `db-support` before starting so we can coordinate.

---

## Commit convention

We use [Conventional Commits](https://www.conventionalcommits.org):
- feat: add MySQL support
- fix: correct nullable inference for @Column without explicit nullable
- docs: update PostgreSQL setup guide
- test: add fixture for MappedSuperclass inheritance
- chore: bump sqlx to 0.7.4

---

## Pull request checklist

Before opening a PR, make sure:

- [ ] `cargo test` passes
- [ ] `cargo clippy` passes with no warnings
- [ ] `cargo fmt` was run
- [ ] New behavior has tests
- [ ] `CHANGELOG.md` has an entry under `Unreleased`

---

## Code of Conduct

This project follows the [Contributor Covenant](https://www.contributor-covenant.org/version/2/1/code_of_conduct/).
Be respectful. Be constructive. That's it.
