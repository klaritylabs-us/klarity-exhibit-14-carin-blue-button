# CARIN Blue Button (EOB) — Rust

Small library and CLI that sanity-check **CARIN-style consumer** **`ExplanationOfBenefit`** JSON before it leaves your payer integration layer: resource type and **`patient.reference`** — the minimum graph needed for member-facing apps to attach claims to a person.

Synthetic fixtures only. Not ONC certification, not a substitute for the HL7 FHIR Validator or Inferno, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 14** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [CARIN Blue Button IG](http://hl7.org/fhir/us/carin-bb/)

## What it checks

- `resourceType` is **`ExplanationOfBenefit`**
- **`patient.reference`** present (pointer `/patient/reference`)

| Problem | What breaks |
|--------|-------------|
| **EOB without patient link** | Member app cannot show “my claims” |
| **Wrong resource type** | Blue Button client parsers skip the row |
| **Patient ref as display only** | Reference graph breaks in Bundle imports |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-carin -- examples/valid-eob.json
cargo run --bin validate-carin -- examples/invalid-carin.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or JSON.

## Sample output

One command:

```bash
cargo run --bin validate-carin -- examples/valid-eob.json
```

**Pass** (`examples/valid-eob.json`) — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-carin.json`) — exit code `1` (issues array; more entries may appear):

```json
[
  {
    "severity": "error",
    "path": "resourceType",
    "detail": "CARIN consumer flows often use ExplanationOfBenefit",
    "rule_id": "carin-eob"
  }
]
```


## Layout

- `src/validate.rs` — rules
- `examples/` — valid and broken EOB payloads for tests and demos

## Inferno vs this repo

Inferno and payer test kits exercise **CARIN IG behavior** on certified endpoints. This repo is **payload shape** — EOB rows your bulk or REST export emits before members download them.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
