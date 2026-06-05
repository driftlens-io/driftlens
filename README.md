# DriftLens

> Schema drift detection for any language, any ORM, any database.

![License](https://img.shields.io/badge/license-AGPL--3.0-blue)
![Status](https://img.shields.io/badge/status-coming%20soon-orange)

---

## The problem

Every production incident starts somewhere. For engineering teams, one of the most silent killers is schema drift — your model says one thing, your database says another, and you only find out when it breaks.

Flyway and Liquibase manage migrations — but neither tells you when the database has **already drifted** from your model. DriftLens does.

---

## What it does

- Detects mismatches between your data model and the live database schema
- Works with any stack — Java, Python, Go, Ruby, .NET and more
- Supports PostgreSQL, MySQL, Oracle, SQL Server and other relational databases
- Runs as a CLI, Docker container, or GitHub Action
- Blocks deploys when critical drift is found
- Generates a human-readable HTML report with full diff detail
- Zero data leaves your environment — works fully offline

Like SonarQube does for code quality, DriftLens does for schema integrity — language-agnostic, pipeline-native, and built to scale with your team.

---

## How it works

1. Reads your data model definitions — source files, bytecode, or schema manifests
2. Connects to your database and introspects the real schema
3. Compares both and generates a detailed drift report
4. Returns a non-zero exit code if critical drift is found — blocking the deploy

---

## Quick start

```bash
docker run --rm \
  -v $(pwd)/src:/app/src \
  -e DATABASE_URL=postgres://user:pass@host:5432/db \
  ghcr.io/driftlens-io/driftlens:latest
```

---

## Supported stacks

| Language | ORM / Framework | Status |
|----------|----------------|--------|
| Java | JPA / Hibernate | 🚧 In progress |
| Python | SQLAlchemy | 🗓️ Planned |
| Go | GORM | 🗓️ Planned |
| Ruby | ActiveRecord | 🗓️ Planned |
| .NET | Entity Framework | 🗓️ Planned |

| Database | Status |
|----------|--------|
| PostgreSQL | 🚧 In progress |
| Oracle | 🚧 In progress |
| MySQL / MariaDB | 🗓️ Planned |
| SQL Server | 🗓️ Planned |

---

## Status

DriftLens is currently under active development.

- [x] Architecture defined
- [ ] PostgreSQL support
- [ ] Java / JPA source parser
- [ ] HTML report
- [ ] GitHub Action
- [ ] Oracle support

Star this repo to follow along.

---

## License

[AGPL-3.0](./LICENSE) — free for open source use.
Commercial and enterprise licenses available at [driftlens.io](https://driftlens.io).
