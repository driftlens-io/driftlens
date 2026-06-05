# Contributing to DriftLens

First off — thank you. DriftLens is early stage and every contribution matters.

---

## Before you start

For anything beyond typos and small fixes, **open an issue first**.
This avoids wasted effort if the change doesn't align with the roadmap.

---

## Local setup

### Requirements

- Rust 1.77+ (`rustup` recommended)
- PostgreSQL 14+ running locally
- `sqlx-cli` for query cache management
- Java 17+ (for running fixture projects — only needed for parser development)

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup component add rustfmt clippy
```

### Install sqlx-cli

`sqlx` validates SQL queries at compile time. `sqlx-cli` generates the query
cache so the project compiles without a live database connection.

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

### Clone and build

```bash
git clone https://github.com/driftlens-io/driftlens
cd driftlens
cargo build
```

> The first `cargo build` may fail if `DATABASE_URL` is not set.
> See the **Database setup** section below to fix this.

---

## Database setup

DriftLens uses `sqlx` query macros that validate SQL at compile time.
This requires either a live database or the offline query cache.

### Option A — Use the offline cache (recommended)

The repository ships with a pre-generated `.sqlx/` cache. This allows
`cargo build` to work without any database connection:

```bash
cargo build
```

If you modify any SQL query in `crates/db-probe/`, regenerate the cache:

```bash
DATABASE_URL=postgres://postgres:postgres@localhost:5432/driftlens_test \
  cargo sqlx prepare --workspace
```

Then commit the updated `.sqlx/` directory.

### Option B — Use a live database

Export `DATABASE_URL` before building:

```bash
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/driftlens_test
cargo build
```

---

## Integration tests

Integration tests connect to a real PostgreSQL database and are not run
by default. They require the `driftlens_test` database with the fixture
schema applied.

### Create the test database

```bash
createdb driftlens_test
psql -d driftlens_test -f tests/fixtures/sql-schemas/postgres/simple.sql
```

### Run integration tests

Integration tests are ignored by default and require a live database:

```bash
# run only integration tests
DATABASE_URL=postgres://postgres:postgres@localhost:5432/driftlens_test \
  cargo test --package driftlens-db-probe -- --ignored --nocapture

# run all tests including integration
DATABASE_URL=postgres://postgres:postgres@localhost:5432/driftlens_test \
  cargo test --package driftlens-db-probe -- --include-ignored --nocapture
```

---

## Project structure

| Directory             | Purpose                                                         |
| --------------------- | --------------------------------------------------------------- |
| `crates/core`         | Shared types — `EntityModel`, `DriftReport`, `DriftKind`        |
| `crates/model-reader` | Reads data model definitions from source files and bytecode     |
| `crates/db-probe`     | Connects to the database and introspects the real schema        |
| `crates/reporter`     | Generates HTML, JSON, and Markdown reports from a `DriftReport` |
| `cli`                 | The main binary — wires everything together                     |
| `action`              | GitHub Action wrapper                                           |
| `tests/fixtures`      | Java projects and SQL schemas used in integration tests         |

---

## How to add support for a new database

This is the most impactful contribution you can make.

1. Create a new module under `crates/db-probe/src/`
2. Implement the `DbProbe` trait defined in `crates/db-probe/src/probe.rs`
3. Add fixture SQL schemas under `tests/fixtures/sql-schemas/<your-db>/`
4. Add integration tests under `crates/db-probe/tests/`
5. Regenerate the sqlx cache: `cargo sqlx prepare --workspace`

Open an issue tagged `db-support` before starting so we can coordinate.

---

## How to add support for a new language or ORM

1. Create a new module under `crates/model-reader/src/`
2. Implement parsing logic that produces `Vec<EntityModel>`
3. Add Java/Python/Go fixture projects under `tests/fixtures/<language>-projects/`
4. Add unit tests alongside the implementation

Open an issue tagged `language-support` before starting so we can coordinate.

---

## Commit convention

We use [Conventional Commits](https://www.conventionalcommits.org):

- feat(db-probe): add MySQL support
- fix(parser): correct nullable inference for @Column without explicit nullable
- docs: update PostgreSQL setup guide
- test: add fixture for MappedSuperclass inheritance
- chore: bump sqlx to 0.7.4

Scope is optional but recommended — use the crate name: `core`, `db-probe`,
`model-reader`, `reporter`, `cli`.

---

## Pull request checklist

Before opening a PR, make sure:

- [ ] `cargo build` passes without `DATABASE_URL` (uses offline cache)
- [ ] `cargo test` passes
- [ ] `cargo clippy` passes with no warnings
- [ ] `cargo fmt --all` was run
- [ ] New SQL queries have an updated `.sqlx/` cache (`cargo sqlx prepare --workspace`)
- [ ] New behavior has tests
- [ ] `CHANGELOG.md` has an entry under `Unreleased`

---

## Code of Conduct

This project follows the [Contributor Covenant](https://www.contributor-covenant.org/version/2/1/code_of_conduct/).
Be respectful. Be constructive. That's it.
