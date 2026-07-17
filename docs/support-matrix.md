# Support and Conformance Matrix

This matrix is the authoritative statement of current `ccsds-ndm` capabilities. Source-code
coverage, examples, fixtures, and passing tests do not establish advertised conformance unless the
exact capability cell below has status `verified`.

There are currently **no advertised conformant capabilities**. The first cell is being developed as
a tracer bullet for the evidence model.

## Status Vocabulary

- `unsupported` — the capability is intentionally unavailable on this surface.
- `planned` — the capability is in scope, but no usable implementation is claimed.
- `implemented-unverified` — an implementation exists and may be useful, but one or more applicable
  quality gates lack evidence. This is not a conformance claim.
- `experimental` — the capability is exposed for evaluation with explicitly unstable behavior or
  interfaces. This is not a conformance claim.
- `verified` — every applicable capability-level gate has auditable normative and executable
  evidence, and every applicable surface/release gate is green. Only this status is advertised as
  conformant.

Statuses describe exact cells; they do not imply support for another edition, notation, operation,
target, or public surface. A status moves to `verified` only in a reviewed change that closes every
listed evidence gap. A regression moves it back to `implemented-unverified` or `unsupported`.

## Evidence Scope

Capability-level evidence establishes the behavior of one exact cell. Depending on the operation,
it includes a normative requirement inventory, strict behavior and any advertised recovery rules,
complete validation, notation-specific output checks, semantic preservation, deterministic output,
diagnostics, and malformed-input or resource-safety tests.

Surface/release evidence establishes that the capability reaches users as tested. It includes API
compatibility, artifact installation, documentation agreement, supported-platform coverage,
security and migration policy, and reproducible releases. Project-wide parity and reference status
are reported separately and cannot be inferred from one verified cell.

For KVN, the schema-revision dimension is `N/A`; its notation evidence instead identifies the exact
standard-derived lexical and field-order requirements. Conversion cells name both source and target
notations and editions.

## Capability Cells

| ID | Message and edition | Standard and corrigendum | Notation / schema | Operation and target | Surface | Status |
| --- | --- | --- | --- | --- | --- | --- |
| `OPM-3.0-XML-GENERATE-RUST` | OPM 3.0 | CCSDS 502.0-B-3 + EC 1 | XML; NDM/XML schema set 4.0.0, OPM schema 3.0 | Generate OPM 3.0 XML | Rust | `implemented-unverified` |

### `OPM-3.0-XML-GENERATE-RUST`

Available evidence:

- Normative source: [Orbit Data Messages, issue 3 and editorial corrigendum 1](ccsds-books/odm.rst)
  and its OPM requirements, including sections 3, 7, 8, and normative annex A.
- Official schema artifact:
  [`ndmxml-4.0.0-opm-3.0.xsd`](../data/xsd/ndmxml-4.0.0-opm-3.0.xsd), reached through the
  [`ndmxml-4.0.0-master-4.0.xsd`](../data/xsd/ndmxml-4.0.0-master-4.0.xsd) schema set.
- Implementation: [`Opm` parsing and generation](../ccsds-ndm/src/messages/opm.rs), shared
  [generation gate](../ccsds-ndm/src/generation.rs), and [XML writer](../ccsds-ndm/src/xml.rs).
- Executable evidence:
  [`opm_3_xml_generation_conformance.rs`](../ccsds-ndm/tests/opm_3_xml_generation_conformance.rs)
  checks every shipped OPM 3.0 KVN and XML fixture against the official schema, checks exact output
  determinism across the Rust generation entry points, and uses public-model mutations to check
  rejection of invalid self-contained states. It also covers every safely reachable OPM
  missing-required-field diagnostic path and compatibility-tests one such diagnostic across every
  Rust generation entry point. Run it with `just conformance-opm-xml`.
- Requirement inventory:
  [`opm-3.0-xml-generation.md`](conformance/opm-3.0-xml-generation.md) maps the applicable ODM and
  project quality requirements to current evidence and explicit gaps.

Evidence still required before `verified`:

- closure of every remaining `Partial` and `Gap` in the requirement inventory;
- stable, structured diagnostic codes and paths for every generation failure;
- panic-free and bounded-resource evidence for adversarial public model states and output failures;
- Rust API documentation and compatibility tests for all advertised generation entry points; and
- the applicable Rust artifact installation, platform, compatibility, security, and reproducible-
  release gates.

## Advertised Capabilities

None. Rows with any other status than `verified` are implementation inventory only.
