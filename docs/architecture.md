# Architecture

```text
Bulk export job completes
  → manifest.json (Bundle + output links)
       → validate_manifest_json()
  → NDJSON object files (one resource per line)
       → validate_ndjson_line() per line
  → Vec<ValidationIssue>  (CLI prints JSON array)
```

| Stage | Failure mode this exhibit catches |
|-------|-----------------------------------|
| Manifest | No `output` link — downloader idle after HTTP 200 |
| Manifest | Wrong Bundle `type` — client ignores manifest |
| NDJSON stream | Blank lines — row counts vs. object counts diverge |
| NDJSON stream | Non-JSON line — entire file rejected by bulk consumers |

| Module | Role |
|--------|------|
| `validate.rs` | manifest + NDJSON rules |
| `outcome.rs` | severity, path, detail, rule_id |
| `bin/validate-bulk.rs` | file input, stdout |

**Related specs:** [Bulk Data IG](https://hl7.org/fhir/uv/bulkdata/), Patient Access async export patterns.

Out of scope: live `$export` polling, auth, object-store ACLs, PHI.
