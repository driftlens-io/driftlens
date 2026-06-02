# Security Policy

## Data Privacy

DriftLens connects directly to your database to introspect schema metadata.

We take this responsibility seriously:

- DriftLens **never transmits schema data, table names, column names, or any database content** to external servers
- The only outbound network request DriftLens makes is license validation, which sends only an anonymous license key — no database information
- License validation can be disabled entirely for air-gapped environments (Enterprise plan)

You can verify this by reviewing the source code or by running DriftLens with network access blocked — drift detection works fully offline.

---

## Supported Versions

| Version | Supported |
|---------|-----------|
| latest  | ✅        |

---

## Reporting a Vulnerability

If you discover a security vulnerability, **please do not open a public GitHub issue**.

Report it privately via GitHub's Security Advisory:
👉 [Report a vulnerability](../../security/advisories/new)

We will respond within **72 hours** and aim to release a fix within **14 days** depending on severity.

---

## Scope

We consider the following in scope for security reports:

- Credential or secret exposure
- Arbitrary code execution
- License bypass
- Data exfiltration of any kind
