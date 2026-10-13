# FHIR Bulk Data `$export` — Rust

Small library and CLI that sanity-check **Bulk Data export manifest** JSON and **NDJSON line** shape before your async export job finishes: manifest **`Bundle`**, **`output`** links, and one JSON object per NDJSON line — plumbing that fails quietly when manifests omit output URLs or streams ship blank lines.

Synthetic fixtures only. Not ONC certification, not a substitute for the HL7 FHIR Validator or Inferno, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 10** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [Bulk Data IG](https://hl7.org/fhir/uv/bulkdata/)
- [FHIR Bundle](http://hl7.org/fhir/R4/bundle.html)

## What it checks

**Manifest (`validate_manifest_json`)**

- `resourceType` is **`Bundle`**
- Warning if `type` is not **`document`** (common bulk manifest pattern)
- At least one `link` with `relation` **`output`** pointing at NDJSON files

**NDJSON (`validate_ndjson_line`)**

- Non-empty trimmed line
- Line parses as a single JSON object (library; covered in unit tests)

| Problem | What breaks |
|--------|-------------|
| **Manifest without `output` link** | Polling succeeds but downloader has nowhere to fetch |
| **Wrong Bundle type** | Client parsers skip or mis-route manifest |
| **Blank NDJSON lines** | Row counts drift; downstream ETL drops partial files |
| **Non-JSON lines in stream** | Entire export file rejected by bulk consumers |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-bulk -- examples/valid-manifest.json
cargo run --bin validate-bulk -- examples/invalid-manifest.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or JSON.

## Sample output

One command:

```bash
cargo run --bin validate-bulk -- examples/valid-manifest.json
```

**Pass** (`examples/valid-manifest.json`) — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-manifest.json`) — exit code `1` (issues array; more entries may appear):

```json
[
  {
    "severity": "error",
    "path": "link",
    "detail": "manifest needs output link to NDJSON files",
    "rule_id": "output-link"
  }
]
```


## Layout

- [docs/architecture.md](./docs/architecture.md) — bulk manifest and NDJSON pipeline
- `src/validate.rs` — manifest and NDJSON rules
- `examples/` — valid and broken manifest JSON for tests and demos

## Inferno vs this repo

Inferno exercises **bulk export workflows** against certified endpoints. This repo is **manifest and stream shape** — the artifacts your job runner writes before files land in object storage.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
