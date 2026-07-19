# Support and Conformance Matrix

This matrix is the authoritative statement of current `ccsds-ndm` capabilities. Source-code
coverage, examples, fixtures, and passing tests do not establish advertised conformance unless the
exact capability cell below has status `verified`.

The exact OPM 3.0 and OEM 3.0 cells below are technically verified for their stated operations,
notations, surfaces, and tested release environments. This does not imply support for another NDM
message, edition, platform, permissive profile, or reference-quality project maturity.

The [OPM 3.0 completion roadmap](opm-completion-roadmap.md) records how parsing, validation,
generation, and conversion were completed across the intended public surfaces. It does not alter
the statuses below.

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
| `OPM-3.0-XML-GENERATE-RUST` | OPM 3.0 | CCSDS 502.0-B-3 + EC 1 | XML; NDM/XML schema set 4.0.0, OPM schema 3.0 | Generate OPM 3.0 XML | Rust | `verified` |
| `OPM-3.0-KVN-GENERATE-RUST` | OPM 3.0 | CCSDS 502.0-B-3 + EC 1 | KVN | Generate OPM 3.0 KVN | Rust | `verified` |
| `OPM-3.0-XML-PARSE-RUST` | OPM 3.0 | CCSDS 502.0-B-3 + EC 1 | XML; NDM/XML schema set 4.0.0, OPM schema 3.0 | Strict parse and self-contained validation | Rust | `verified` |
| `OPM-3.0-KVN-PARSE-RUST` | OPM 3.0 | CCSDS 502.0-B-3 + EC 1 | KVN | Strict parse and self-contained validation | Rust | `verified` |
| `OPM-3.0-VALIDATE-RUST` | OPM 3.0 | CCSDS 502.0-B-3 + EC 1 | N/A | Validate typed OPM model | Rust | `verified` |
| `OPM-3.0-KVN-TO-XML-RUST` | OPM 3.0 | CCSDS 502.0-B-3 + EC 1 | KVN → XML 4.0.0 / OPM 3.0 | Convert, preserving edition and meaning | Rust | `verified` |
| `OPM-3.0-XML-TO-KVN-RUST` | OPM 3.0 | CCSDS 502.0-B-3 + EC 1 | XML 4.0.0 / OPM 3.0 → KVN | Convert, preserving edition and meaning | Rust | `verified` |
| `OPM-3.0-XML-PARSE-PYTHON` | OPM 3.0 | CCSDS 502.0-B-3 + EC 1 | XML; NDM/XML schema set 4.0.0, OPM schema 3.0 | Strict parse and self-contained validation | Python | `verified` |
| `OPM-3.0-KVN-PARSE-PYTHON` | OPM 3.0 | CCSDS 502.0-B-3 + EC 1 | KVN | Strict parse and self-contained validation | Python | `verified` |
| `OPM-3.0-XML-GENERATE-PYTHON` | OPM 3.0 | CCSDS 502.0-B-3 + EC 1 | XML; NDM/XML schema set 4.0.0, OPM schema 3.0 | Generate OPM 3.0 XML | Python | `verified` |
| `OPM-3.0-KVN-GENERATE-PYTHON` | OPM 3.0 | CCSDS 502.0-B-3 + EC 1 | KVN | Generate OPM 3.0 KVN | Python | `verified` |
| `OPM-3.0-VALIDATE-PYTHON` | OPM 3.0 | CCSDS 502.0-B-3 + EC 1 | N/A | Validate typed OPM model | Python | `verified` |
| `OPM-3.0-KVN-TO-XML-PYTHON` | OPM 3.0 | CCSDS 502.0-B-3 + EC 1 | KVN → XML 4.0.0 / OPM 3.0 | Convert | Python | `verified` |
| `OPM-3.0-XML-TO-KVN-PYTHON` | OPM 3.0 | CCSDS 502.0-B-3 + EC 1 | XML 4.0.0 / OPM 3.0 → KVN | Convert | Python | `verified` |
| `OPM-3.0-VALIDATE-CLI` | OPM 3.0 | CCSDS 502.0-B-3 + EC 1 | KVN or XML 4.0.0 / OPM 3.0 | Validate input | CLI | `verified` |
| `OPM-3.0-KVN-TO-XML-CLI` | OPM 3.0 | CCSDS 502.0-B-3 + EC 1 | KVN → XML 4.0.0 / OPM 3.0 | Convert | CLI | `verified` |
| `OPM-3.0-XML-TO-KVN-CLI` | OPM 3.0 | CCSDS 502.0-B-3 + EC 1 | XML 4.0.0 / OPM 3.0 → KVN | Convert | CLI | `verified` |
| `OEM-3.0-XML-GENERATE-RUST` | OEM 3.0 | CCSDS 502.0-B-3 + EC 1 | XML; NDM/XML schema set 4.0.0, OEM schema 3.0 | Generate OEM 3.0 XML | Rust | `verified` |
| `OEM-3.0-KVN-GENERATE-RUST` | OEM 3.0 | CCSDS 502.0-B-3 + EC 1 | KVN | Generate OEM 3.0 KVN | Rust | `verified` |
| `OEM-3.0-XML-PARSE-RUST` | OEM 3.0 | CCSDS 502.0-B-3 + EC 1 | XML; NDM/XML schema set 4.0.0, OEM schema 3.0 | Strict parse and self-contained validation | Rust | `verified` |
| `OEM-3.0-KVN-PARSE-RUST` | OEM 3.0 | CCSDS 502.0-B-3 + EC 1 | KVN | Strict parse and self-contained validation | Rust | `verified` |
| `OEM-3.0-VALIDATE-RUST` | OEM 3.0 | CCSDS 502.0-B-3 + EC 1 | N/A | Validate typed OEM model | Rust | `verified` |
| `OEM-3.0-KVN-TO-XML-RUST` | OEM 3.0 | CCSDS 502.0-B-3 + EC 1 | KVN → XML 4.0.0 / OEM 3.0 | Convert, preserving edition and meaning | Rust | `verified` |
| `OEM-3.0-XML-TO-KVN-RUST` | OEM 3.0 | CCSDS 502.0-B-3 + EC 1 | XML 4.0.0 / OEM 3.0 → KVN | Convert, preserving edition and meaning | Rust | `verified` |

### Implemented family plumbing that is not yet verified

The following is an implementation inventory, not a set of advertised capability cells. Each row
summarises registration through the shared Rust core, Python adapters, and CLI for the listed
current edition. The grouped status must not be read as verification of every operation, notation,
or surface: exact cells will be added and promoted only after their message-specific normative and
executable inventories are complete.

| Message / current edition | Implemented shared surface | Status | Principal evidence gap |
| --- | --- | --- | --- |
| OPM 2.0 | KVN/XML parse, validate, generate, convert; Rust/Python/CLI | `implemented-unverified` | Edition rules and XML generation are checked against the archived official NDM/XML 2.0 schema; exact cross-surface cells and standard-derived KVN evidence remain |
| OEM 2.0 | KVN/XML parse, validate, generate, convert; Rust/Python/CLI | `implemented-unverified` | Edition rules and XML generation are checked against the archived official NDM/XML 2.0 schema; exact cross-surface cells and standard-derived KVN evidence remain |
| OMM 2.0 | KVN/XML parse, validate, generate, convert; Rust/Python/CLI | `implemented-unverified` | Edition-specific TLE requirements and XML generation are checked against the archived official NDM/XML 2.0 schema; exact cross-surface cells and standard-derived KVN evidence remain |
| OMM 3.0 | KVN/XML parse, validate, generate, convert; Rust/Python/CLI | `implemented-unverified` | Exact operation/notation/surface cell review remains; all 70 ICS items, strictness, typed preservation, official-XSD generation, fixed-path allocation budgets, and packaged-surface gates are reconciled |
| OCM 3.0 | KVN/XML parse, validate, generate, convert; Rust/Python/CLI | `implemented-unverified` | Grouped 259-row ICS reconciliation, strict block/XML structure, fixture/nested-block preservation, official-XSD generation, loss rejection, three-history allocation/scaling, Python, CLI, and shared artifact evidence are green; reviewed exact operation/notation/surface cells remain |
| CDM 1.0 | KVN/XML parse, validate, generate, convert; Rust/Python/CLI | `implemented-unverified` | All 68 ICS rows are grouped and reconciled with strict structure, typed fixture preservation, official-XSD generation, loss-rejecting delimiter-free COMMENT handling, Python, CLI, and shared artifact evidence; reviewed exact cells remain |
| TDM 2.0 | KVN/XML parse, validate, generate, convert; Rust/Python/CLI | `implemented-unverified` | Exact operation/notation/surface cell review remains; all 53 ICS items, strictness, full fixture/XSD preservation, optional-unit normalization, history allocation, packaged surfaces, and recorded Criterion scaling are reconciled |
| RDM 1.0 | KVN/XML parse, validate, generate, convert; Rust/Python/CLI | `implemented-unverified` | Exact operation/notation/surface cell review remains; all 142 ICS items, strictness, logical-block comment/unit preservation, official-XSD generation, fixed-path allocation budgets, Criterion, and packaged surfaces are reconciled |
| AEM 2.0 | KVN/XML parse, validate, generate, convert; Rust/Python/CLI | `implemented-unverified` | All 25 ICS rows are grouped with strict structure, all-nine-choice preservation, timeline semantics, fixed-unit normalization, official-XSD, allocation/scaling, Python, CLI, and artifact evidence; the official G4 degree-7/four-record conflict and reviewed exact cells remain |
| APM 2.0 | KVN/XML parse, validate, generate, convert; Rust/Python/CLI | `implemented-unverified` | Exact operation/notation/surface cell review remains; all 82 ICS items, strictness, choice preservation, official-XSD generation, fixed-path allocation budgets, Criterion, and packaged surfaces are reconciled |
| ACM 2.0 | KVN/XML parse, validate, generate, convert; Rust/Python/CLI | `implemented-unverified` | All 110 ICS rows are grouped with strict block/XML structure, schema-model corrections, typed fixture preservation, official-XSD generation, loss rejection, allocation/scaling, Python, CLI, and artifact evidence; reviewed exact cells remain |
| Combined NDM envelope | Strict XML parse/generate and generic Rust/Python/CLI dispatch; sequential KVN convenience form | `implemented-unverified` | Sections 4.11.3–4.11.8 are reconciled with exact-envelope, ordered-child, official-schema, aggregate-resource, surface, artifact, and allocation evidence; exact-cell review and the XSD-valid G22 versus OPM maneuver/MASS conflict remain |

The [shared family contract inventory](conformance/family-shared-contract.md) links the reusable
resource, diagnostic, dispatch, and Criterion/CodSpeed evidence. OPM and OEM retain their exact
verified cells above; this section records only the remaining implementation state and does not
downgrade or broaden those claims.

The [OMM 3.0 focused inventory](conformance/omm-3.0.md) records the first message-specific slice
beyond the shared family plumbing: strict KVN layout, strict nested XML sequences and attributes,
typed-model preservation for all shipped fixtures, and official-schema generation evidence.

Focused inventories for [OCM 3.0](conformance/ocm-3.0.md),
[CDM 1.0](conformance/cdm-1.0.md),
[AEM 2.0](conformance/aem-2.0.md),
[TDM 2.0](conformance/tdm-2.0.md),
[RDM 1.0](conformance/rdm-1.0.md), [APM 2.0](conformance/apm-2.0.md), and
[ACM 2.0](conformance/acm-2.0.md), plus the
[Combined NDM envelope](conformance/combined-ndm.md), record their current
message-specific strictness, preservation, official-schema, resource, and performance evidence
without promoting unverified cells.

### OEM 3.0 Rust evidence

The [OEM 3.0 conformance inventory](conformance/oem-3.0.md) maps the seven Rust cells to the
applicable ODM sections, official OEM 3.0 XSD, implementation boundaries, and focused executable
evidence. `oem_strict_parsing`, `oem_parse_diagnostics`, `oem_validation`,
`oem_generation_conformance`, `oem_conversion`, and `oem_kvn_allocations` cover all shipped OEM
fixtures, strict rejection, semantic validation, structured diagnostics, official-schema output,
lossless normalized conversion, resource limits, atomic files, and stable streaming allocation
behavior. Existing `kvn_scaling` and `xml_scaling` benchmarks provide
reproducible 10–50,000-record workloads through `just bench-oem`; wall-clock results are
informational. `just verify-oem` reproduces the complete Rust technical and packaged-artifact
evidence. No OEM Python or CLI capability is advertised.

### Parsing, validation, conversion, Python, and CLI evidence

- [KVN parsing inventory](conformance/opm-3.0-kvn-parsing.md) and
  [XML parsing inventory](conformance/opm-3.0-xml-parsing.md) map strict syntax, preservation,
  diagnostics, and resource behavior.
- The [typed-model validation inventory](conformance/opm-3.0-validation.md) separates
  self-contained model rules, notation-boundary rules, and caller-context semantics.
- The [conversion inventory](conformance/opm-3.0-conversion.md) covers both notation directions,
  complete typed equivalence, loss rejection, limits, and atomic files.
- The [Python and CLI surface inventory](conformance/opm-3.0-surfaces.md) maps each adapter cell to
  focused tests and the exact packaged release scope.
- `opm_conversion`, `opm_parse_diagnostics`, `opm_parse_limits`, `opm_strict_kvn_parsing`, and
  `opm_strict_xml_parsing` are the focused Rust evidence.
- Python OPM parsing/generation/validation delegates to Rust and exposes structured exception
  attributes and the same resource policies; `test_parse_and_generation_options.py` exercises the
  parity contract.
- `opm_cli` fixes CLI exits, JSON diagnostics, stdin/stdout separation, conversion, resource limits,
  and atomic destination behavior.

Tracked OPM KVN/XML seeds exercise the existing panic-safety fuzz targets. `just package-python`
builds the ABI3 wheel, installs it with dependencies into an isolated environment, and imports the
OPM and conversion APIs. `just verify-opm` reproduces the complete quality, focused conformance,
benchmark-build, documentation, and installed-artifact evidence. Independent adoption, mature
private security handling, broader platform operation, sustained fuzzing, and stable wall-clock
thresholds remain reference-status work and are not implied by these technical cells.

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
  [`opm_3_xml_generation_conformance.rs`](../ccsds-ndm/tests/opm_3_xml_generation_conformance.rs),
  [`opm_keplerian_xml_generation.rs`](../ccsds-ndm/tests/opm_keplerian_xml_generation.rs), and
  [`opm_maneuver_duration_units.rs`](../ccsds-ndm/tests/opm_maneuver_duration_units.rs) check every
  shipped OPM 3.0 KVN and XML fixture against the official schema, exact output determinism across
  the Rust generation entry points, and rejection of invalid self-contained public-model states.
  Together they cover every safely reachable OPM missing-required-field, invalid-choice,
  invalid-value, and out-of-range diagnostic path, plus unsupported-output-version and file-I/O
  codes. The same suite fixes the documented public Rust generation signatures, while the raw
  serde XML serializer remains crate-internal so callers cannot bypass generation validation.
  [`opm_xml_writer_failure.rs`](../ccsds-ndm/tests/opm_xml_writer_failure.rs) covers streaming I/O
  diagnostics and panic-free propagation at multiple output boundaries. Run the main cell with
  `just conformance-opm-xml`.
- Performance evidence:
  [`xml_benches.rs`](../ccsds-ndm/benches/xml_benches.rs) benchmarks the richest shipped OPM
  fixture through validated materialized and streaming XML generation. Run it with
  `just bench-opm-xml`. Wall-clock results are informational; deterministic allocation and output
  budgets are enforced.
- Rust surface/release evidence:
  the [pre-1.0 Rust release policy](rust-release-policy.md) records the exact tested toolchain and
  platform scope, unstable compatibility contract, migration expectation, and public-only security
  reporting. `just package-rust` and the clean-checkout CI packaging job exercise Cargo's
  package selection and verify the unpacked build before publishing; the release job also rejects
  a tag that does not match the crate version.
- Requirement inventory:
  [`opm-3.0-xml-generation.md`](conformance/opm-3.0-xml-generation.md) maps the applicable ODM and
  project quality requirements to current evidence.

All applicable technical and proportionate pre-1.0 surface gates are covered. Reference-status
maturity is tracked separately.

### `OPM-3.0-KVN-GENERATE-RUST`

Available evidence:

- Normative source: ODM sections 3.2 and 7.3–7.9 define OPM content, fixed KVN ordering, lexical
  rules, units, and comment placement.
- Executable evidence:
  [`opm_3_kvn_generation_conformance.rs`](../ccsds-ndm/tests/opm_3_kvn_generation_conformance.rs)
  verifies all four shipped Annex G fixtures retain assignment order, comments, and optional units;
  generated representative lines are printable ASCII and no longer than 254 characters. It also
  covers CCSDS-bounded ODM numeric spelling and pre-write rejection of invalid free text, non-finite numeric
  values, and the normative uppercase/no-blank rule for user-defined keyword suffixes without
  turning Annex F's recommended regex into a mandatory restriction. Focused streaming sink failures
  verify panic-free preservation of I/O diagnostics. The richest numeric/covariance fixture also
  has enforced allocation budgets for materialized and pre-sized streaming generation. Run the
  evidence with `just conformance-opm-kvn`; reproduce the timing and repeated-maneuver scaling
  benchmarks with `just bench-opm-kvn`.
- Requirement inventory:
  [`opm-3.0-kvn-generation.md`](conformance/opm-3.0-kvn-generation.md) maps current evidence.

Structured generation context, aggregate output bounds, zero-byte streaming preflight rejection,
sink failures, KVN allocation budgets, and current-stable Ubuntu artifact installation have
executable evidence. Wall-clock thresholds are optional under the proportionate pre-1.0 policy.

## Advertised Capabilities

The 17 `verified` OPM 3.0 rows and seven `verified` OEM 3.0 Rust rows above are the advertised
capabilities. No capability is advertised beyond those exact cells and their documented tested
release environments.
