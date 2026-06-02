# DriftLens

> Detects JPA schema drift before it hits production.

![License](https://img.shields.io/badge/license-AGPL--3.0-blue)
![Status](https://img.shields.io/badge/status-coming%20soon-orange)

---

## The problem

Every team using JPA/Hibernate in production has been bitten by schema drift.

`ddl-auto=update` silently creates columns, drops nothing, and hides divergence between your Java model and the real database. Flyway and Liquibase manage migrations — but neither tells you when the database has already drifted from your model.

DriftLens does.

---

## How it works

1. Reads your JPA entities from source files or compiled bytecode
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

## Status

DriftLens is currently under active development.

- [x] Architecture defined
- [ ] PostgreSQL support
- [ ] JPA source parser
- [ ] HTML report
- [ ] GitHub Action
- [ ] Oracle support

Star this repo to follow along.

---

## License

[AGPL-3.0](./LICENSE) — free for open source use.
Commercial and enterprise licenses available at [driftlens.io](https://driftlens.io).
