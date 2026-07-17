# Rust core review: correctness, tolerance, performance, and quality

Review date: 2026-07-14

## Executive assessment

The Rust core has a promising base: broad NDM coverage, typed message models, dedicated KVN parsers, `quick-xml`, `winnow`, a sizeable fuzz corpus, and useful integration fixtures. It is not yet safe to describe as a reference-quality implementation, however. At review time, the design could emit schema-invalid XML, emitted KVN without validation, silently discarded malformed OCM records, and had no streaming API for the largest message types. Parser tolerance was inconsistent and usually silent rather than diagnostic.

The most important product rule should be:

> Parsing may recover from explicitly documented deviations, but recovery must never be silent. Generation must accept only validated state and must produce a selected, fully supported CCSDS edition.

That rule was not enforced when the review was performed. The decisions section records the first
implementation steps completed afterward; unresolved findings below still apply unless explicitly
noted.

## Review scope and method

This review covers the Rust core only. It does not establish parity of the Python
bindings, Python documentation, or Python tests; those remain a separate review
and release requirement. The review uses the applicable published CCSDS standards
as the conformance authority. The supplied XSDs, fixtures, and other
implementations are evidence and interoperability references, not substitutes for
the standards.

- Traced every `Ndm` parse and generation entry point and the shared validation, detection, XML, KVN, and primitive-type layers.
- Compared the XML models and emitted field order/names with the applicable supplied schemas in `data/xsd/`.
- Generated XML from all available KVN fixtures and regenerated all available XML fixtures: 65 transformations in total. Three generated documents failed their applicable supplied XSD.
- Ran focused probes for XML namespaces, KVN header detection, legacy version generation, and malformed OCM trajectory rows.
- Reviewed the benchmark and fuzz targets for what they prove, not just whether they exist.

This was an architecture and source review, not a statistically rigorous performance study. Performance findings below identify avoidable allocations, copies, and API constraints; they do not claim measured throughput regressions unless explicitly stated.

## Decisions adopted after this review

- Parsing is currently strict-only. A permissive surface will be introduced only with explicit,
  deterministic recovery rules and structured diagnostics.
- Generation of a parsed message preserves its source edition when that edition has a complete,
  conforming serializer. Newly constructed messages target the latest fully supported edition.
  Callers can request an exact edition without adding format-specific method families.
- A serializer refuses an edition it does not explicitly support; accepting an edition while
  parsing does not imply that the library can generate that edition.
- Historical generation is supported only for an edition with a complete, edition-correct
  serializer and demonstrated interoperability need.
- Arbitrary vendor extensions outside CCSDS-defined extension mechanisms are unsupported. They are
  rejected with a located diagnostic and are never silently discarded or regenerated. A
  permissive profile may recover only documented, unambiguous CCSDS deviations.
- Every advertised message type, edition, and notation is recorded in a support/conformance matrix
  linking normative requirements to implementation and tests. A combination is not advertised as
  supported until the matrix shows the full project quality bar is met.
- Generated output is always validated. The Python compatibility argument `validate=False` is
  rejected rather than allowing unchecked output.

## Validation results

- `cargo test`: passed. This ran 566 unit tests, 25 integration tests, and 12 doctests.
- `cargo test --test xsd_schema_validation -- --nocapture`: passed while printing `xmllint not available; skipping XSD validation` ten times. No XSD was checked by that test in this environment.
- `cargo clippy --all-targets --all-features -- -D warnings`: failed with 43 errors in library tests and integration tests.
- The Clippy failures include repeated `false || ...` boolean expressions in OEM tests, redundant borrows, obsolete `map_or(false, ...)` patterns, a useless `format!`, and other mechanical quality defects. They are mostly not runtime defects, but the volume supports the maintainability concern in M7 and means the strict lint gate is not currently green.

## Severity model

- **Blocker**: can lose accepted data or generate a message known not to comply.
- **High**: prevents the stated tolerance, performance, or correctness guarantees.
- **Medium**: important quality, maintainability, or coverage weakness.

## Blockers

### B1. Generation is not guarded by complete validation

Every KVN `to_kvn` implementation writes the public model directly. Representative entry points are `ccsds-ndm/src/messages/opm.rs:53`, `ccsds-ndm/src/messages/oem.rs:176`, `ccsds-ndm/src/messages/ocm.rs:59`, and `ccsds-ndm/src/messages/ndm.rs:67`. Public fields can be mutated or constructed without the deserializers, so a valid parse path does not make later generation safe.

Most XML generators call semantic validation, but CDM and combined NDM do not: `ccsds-ndm/src/messages/cdm.rs:69` and `ccsds-ndm/src/messages/ndm.rs:130`.

Even where `validate()` is called, it is not equivalent to schema validation: it returns the first hand-written semantic error and does not prove required element order, spelling, version-specific availability, unit correctness, or all finite-number constraints.

**Required change:** make generation operate on a `Validated<T>`/canonical representation, or perform complete validation inside every public generation entry point. KVN and XML must share the same pre-generation gate. Provide an explicitly named unchecked internal path only if benchmarks demonstrate it is necessary.

### B2. Supplied fixtures already produce schema-invalid XML

Independent validation of the 65 fixture transformations found three failures:

1. `acm_g7.kvn` and `acm_g9.kvn` generate `<sensor>`, while the ACM 2.0 schema requires `<sensorData>`. The incorrect serde name is at `ccsds-ndm/src/messages/acm.rs:1655`; the schema declaration is at `data/xsd/ndmxml-4.0.0-acm-2.0.xsd:242`.
2. `tdm_e17.kvn` emits `EPHEMERIS_NAME_1` after the correction fields. The XSD requires it before `TRANSMIT_BAND` at `data/xsd/ndmxml-4.0.0-tdm-2.0.xsd:124`. The Rust field is declared much later in `ccsds-ndm/src/messages/tdm.rs:856`, so serde XML follows the wrong order. The hand-written KVN writer separately emits the correct order at `ccsds-ndm/src/messages/tdm.rs:1193`.

This demonstrates that Rust struct declaration order is currently an accidental part of the XML wire format.

**Required change:** fix both mappings and add an exhaustive fixture matrix that validates KVN-to-XML and XML-to-XML output against XSD. Prefer explicit XML serializers for order-sensitive schema sequences, or generate/order-check the model from a machine-readable schema inventory.

### B3. Legacy input versions are accepted but generation is not version-aware

`ccsds-ndm/src/versioning.rs:20` accepts several historical editions, including OPM/OEM 1.0 through 3.0 and AEM/APM/ACM/TDM 1.0 and 2.0. There is only one model and one serializer per message type. A probe that changed a valid OPM fixture to version 1.0 parsed successfully and regenerated a current-shape document still labelled `version="1.0"`.

The supplied current schemas fix the version, for example OPM 3.0 at `data/xsd/ndmxml-4.0.0-opm-3.0.xsd:51`, ACM 2.0 at `data/xsd/ndmxml-4.0.0-acm-2.0.xsd:45`, and TDM 2.0 at `data/xsd/ndmxml-4.0.0-tdm-2.0.xsd:52`. KVN generation has the same problem: newer fields can be written under an older header.

**Required change:** parse supported historical editions into the canonical model and preserve the
source edition by default only when the library has a complete edition-specific validator and
serializer backed by the standard. Otherwise generation must fail and require the caller to select
a fully supported target edition explicitly. Never label current-shape output with an older edition.
If changing editions would lose information or require inventing required data, generation must
fail with a precise diagnostic.

### B4. Malformed OCM records are silently discarded

The OCM trajectory parser tries a row parser and, on any failure, consumes the whole line without a diagnostic at `ccsds-ndm/src/kvn/ocm.rs:275` and `ccsds-ndm/src/kvn/ocm.rs:290`. The maneuver parser uses the same pattern at `ccsds-ndm/src/kvn/ocm.rs:697`.

A focused probe replaced one number in a valid trajectory row with `NOT_A_FLOAT`. Parsing succeeded, regeneration succeeded, and the entire row was absent from the result. This is data loss, not merciful parsing.

**Required change:** recover only for a precisely recognized continuation grammar. Otherwise return
a located parsing error. Diagnostic inspection may retain the raw record as uninterpreted content
while reporting it. A reference-quality implementation must never silently drop an unrecognized
data line.

### B5. XSD tests can pass without performing validation

`ccsds-ndm/tests/xsd_schema_validation.rs:18` depends on an external `xmllint`. If it is absent, the test prints a message and returns successfully at `ccsds-ndm/tests/xsd_schema_validation.rs:34`. That happened in this review environment. The test covers only one fixture for most message types, omits ACM, and does not validate KVN-to-XML generation. Consequently, the failures in B2 are invisible to CI configurations without `xmllint`.

**Required change:** make XSD validation a mandatory test dependency or a required CI tool, and fail if it is unavailable. Keep that dependency in test/CI infrastructure; the parsing and generation runtime must remain offline and independent of an XSD engine. Cover every shipped XML and KVN fixture, all message types, combined NDM, nullable values, optional blocks, and each supported output edition.

### B6. Advertised support is not backed by an explicit conformance matrix

The conformance policy defines support as an operation- and surface-specific promise. The review
identifies individual fixture and serializer defects, but it does not establish one authoritative
inventory of which capabilities are advertised or link each capability to normative requirements
and tests.

**Required change:** maintain a support/conformance matrix for every message type, edition, and
notation. For each advertised combination, link the applicable CCSDS requirements, parsing and
any advertised recovery coverage, validation coverage, KVN/XML generation evidence, cross-notation
semantic round trips, malformed-input/resource tests, and streaming equivalence where applicable.
Do not advertise a combination while any required evidence is missing.

## High-priority findings

### H1. Compatible parsing reports at most one semantic violation

`validate_with_mode` invokes `value.validate()` once at `ccsds-ndm/src/validation.rs:67`. Validators return on their first error. Lenient mode converts only that first error into one warning and then accepts the object at `ccsds-ndm/src/validation.rs:81`; remaining violations are never inspected.

Diagnostics are stored in thread-local global state at `ccsds-ndm/src/validation.rs:36` and remain there until the caller remembers to drain them. This makes warnings easy to misassociate with a later parse and prevents clean async/task-oriented use.

**Required change:** validation should collect all issues into the parse result:

```text
ParseReport<T> { message: T, diagnostics: Vec<Diagnostic> }
```

Diagnostics need severity, format, message type/version, field path, line/column or XML path, original token, recovery action, and a stable code. Strictness should decide which diagnostics reject the parse, not whether validation stops early.

### H2. Tolerance is inconsistent across formats and often silent

- KVN block parsers generally reject unknown keys immediately.
- Serde XML structs generally ignore unknown fields without reporting them.
- TDM's custom XML visitor explicitly ignores unknown content at `ccsds-ndm/src/messages/tdm.rs:1570`.
- Combined XML ignores unknown child elements at `ccsds-ndm/src/messages/ndm.rs:211`.
- Required numeric XML elements may omit units and receive defaults; KVN required numeric wrappers discard the supplied unit entirely at `ccsds-ndm/src/types.rs:453`.

Merciful parsing should be policy-driven and observable. Silent acceptance makes typoed standard fields indistinguishable from supported vendor extensions; hard rejection of the same construct in the other notation is unpredictable.

Arbitrary vendor extensions outside CCSDS-defined extension mechanisms are out of scope. They must
not be preserved or regenerated as an implicit permissive feature. Unknown fields should therefore
produce a located diagnostic and rejection unless they belong to an explicitly modeled CCSDS
extension mechanism.

**Required change:** keep ordinary parsing strict and coherent. If a permissive profile is later
justified by real fixtures, it may apply only enumerated, deterministic recoveries of documented
CCSDS deviations and must report each one. It must not accept, silently drop, or silently preserve
arbitrary vendor extensions. Do not expose independent policy switches without demonstrated need.

### H3. Namespace-qualified XML is rejected

The XSD set declares `urn:ccsds:schema:ndmxml`, but detection lowercases the entire qualified name and compares it to bare names at `ccsds-ndm/src/detect.rs:137`. A valid-looking `<ndm:opm>` probe was not recognized and failed while scanning its descendants. The detector also errors on ordinary end events encountered during that recursive scan.

**Required change:** compare XML local names without allocating, verify or tolerate the namespace according to policy, and parse the actual root rather than recursively searching arbitrary descendants by default. Non-standard wrappers can remain an explicit compatible-mode recovery with a warning.

### H4. Combined KVN detection is substring-based

`detect_kvn_type` counts every occurrence of each header token anywhere in the input at `ccsds-ndm/src/detect.rs:85`. `CombinedNdm::from_kvn` similarly uses `match_indices` at `ccsds-ndm/src/messages/ndm.rs:89`. A probe adding `CCSDS_OEM_VERS` as prose inside an OPM `COMMENT` caused the input to be treated as combined and fail with an OEM header error.

Besides correctness, detection scans the full input once per supported header and scans again when splitting.

**Required change:** reuse the KVN lexer and recognize headers only as top-level keys at record boundaries. Produce slices in one pass.

### H5. Large-message APIs require full materialization

The public trait accepts `&str` and returns `String` at `ccsds-ndm/src/traits.rs:63` and `ccsds-ndm/src/traits.rs:82`. File helpers therefore read or build full documents rather than offering incremental I/O. Large OEM, OCM, AEM, ACM, and TDM workloads simultaneously retain input, a fully owned model, and output.

**Required change:** add:

- strict `from_reader(BufRead)` plus an explicit permissive variant, and `write_kvn(Write, GenerateOptions)` / `write_xml(Write, GenerateOptions)`;
- record iterators or callbacks for high-volume histories;
- a clear owned versus borrowed parsing story;
- resource limits at the I/O boundary, where they can prevent allocation rather than merely reject an already materialized string.

Keep the current convenience APIs as wrappers around the streaming core.

### H6. Optional XML fields take a JSON allocation detour

The shared optional deserializer imports `serde_json` at `ccsds-ndm/src/utils.rs:49`, parses scalar XML text as JSON at `ccsds-ndm/src/utils.rs:114`, buffers XML maps into `serde_json::Map`, clones them at `ccsds-ndm/src/utils.rs:166`, and retries multiple deserialization shapes. It is attached to hundreds of fields, especially OCM, common types, ACM, TDM, and CDM.

This is a poor hot-path design for XML: strings, maps, JSON values, clones, and retry parsing are used to interpret a simple nullable element.

**Required change:** replace it with format-native typed visitors or direct `quick-xml` event handling. Benchmark representative nullable-heavy OCM/ACM/CDM documents, not only OEM state vectors.

### H7. Hot numeric paths are inconsistently optimized

OEM uses dedicated `winnow`/fast-float row parsing, while AEM parses rows with `split_whitespace`, standard `str::parse::<f64>`, and a growing `Vec` at `ccsds-ndm/src/kvn/aem.rs:114`. ACM repeats this pattern at `ccsds-ndm/src/kvn/acm.rs:114`. OCM trajectory rows allocate an epoch `String` and a variable `Vec` at `ccsds-ndm/src/kvn/ocm.rs:213`.

Generation also allocates unnecessarily: `KvnWriter::write_pair` calls `to_string()` for every value at `ccsds-ndm/src/kvn/ser.rs:52`, and XML serialization builds a body string then copies it into a second string with `format!` at `ccsds-ndm/src/xml.rs:55`.

**Required change:** define one numeric token parser/formatter strategy, reserve exact fixed-width vectors, write values directly, and stream the XML declaration and body into one sink. Add allocation counts and bytes allocated to benchmarks.

### H8. Fixed output buffers and unchecked non-finite floats threaten generation

`StateVectorAcc::write_kvn` uses a fixed 256-byte stack buffer at `ccsds-ndm/src/common.rs:550` and appends an epoch plus up to nine formatted floats through unchecked slice ranges. The epoch type permits 64 bytes. A legal-length epoch plus maximum formatted finite values can exceed the fixed buffer and panic.

The same code calls `format_finite` without first proving values are finite at `ccsds-ndm/src/common.rs:563`; OEM covariance output does likewise at `ccsds-ndm/src/messages/oem.rs:695`. Public fields allow callers to supply NaN or infinity, while KVN generation does not validate them.

**Required change:** validate all generated numerics as finite, use checked/growable output, and add property tests over finite `f64` extremes, NaN/infinity, and maximum epoch length. Generation must return an error, never panic or silently substitute a number.

### H9. The epoch type conflates absolute and relative time

`Epoch` accepts either a calendar-like token or a numeric token at `ccsds-ndm/src/types.rs:54`. Its numeric grammar also accepts an empty string, and the calendar grammar checks digit counts but not valid month/day/hour/minute/second/time-zone ranges. `Epoch::new` explicitly accepts empty input at `ccsds-ndm/src/types.rs:104`.

This mirrors permissive XSD lexical patterns but not the stronger field semantics in the CCSDS prose. One type is being used for absolute epochs and relative time tags with different legal domains.

**Required change:** separate a validated `CalendarEpoch` from contextual numeric/calendar
`Epoch`, perform semantic calendar validation, and represent missing optional values with `Option`
rather than an empty epoch. Preserve the original lexical spelling only where round-trip fidelity
requires it. The
validated wrappers and the `OdmHeader.creation_date`/`AdmHeader.creation_date` migrations now
address this finding for the shared ODM and ADM headers. The OCM metadata reference epochs
(`EPOCH_TZERO`, previous/next message epochs, and `NEXT_LEAP_EPOCH`) are also now typed as
`CalendarEpoch`, as are its four nested frame-reference epochs. OMM `REF_FRAME_EPOCH` and the
mean-elements `EPOCH` are now also typed as `CalendarEpoch`. The remaining message families and
contextual OCM fields are still audited incrementally. OPM reference-frame, state-vector, and
maneuver ignition epochs are now also typed as `CalendarEpoch`.

### H10. Unit recovery can silently change meaning

Required numeric XML wrappers accept a scalar without units and inject a default at `ccsds-ndm/src/types.rs:392`. Their KVN conversion ignores the parsed unit token entirely at `ccsds-ndm/src/types.rs:453`. If a producer supplies a non-default unit, the numeric value can be interpreted as the default without conversion or warning.

**Required change:** parse units into a known enum, convert only when the standard permits it, retain approved extension units if policy allows, and emit a diagnostic whenever a default is inferred. Unknown or dimensionally incompatible units must not be silently accepted.

## Medium-priority findings

### M1. Models do not make invalid states hard to represent

Many value and message fields are public. Range-checked constructors and deserializers can be bypassed, derived `Serialize` can emit bypassed values, and root validators do not comprehensively traverse every primitive. This undermines the type-safety claim.

Use private fields plus checked constructors/builders for constrained primitives, or clearly distinguish unchecked DTOs from validated canonical models. Builders should return `Result` when invariants are cross-field or edition-dependent.

### M2. OEM capacity estimation can greatly over-allocate

`oem_data` estimates state records from the entire remaining byte length and reserves `input.len() / 80` entries at `ccsds-ndm/src/kvn/oem.rs:459`. Large comments, covariance blocks, or later segments can cause substantial unnecessary allocation.

Cap the heuristic, estimate only the current section, or grow geometrically from a conservative capacity. Add adversarial memory tests.

### M3. Existing round trips are mostly self-consistency checks

Parsing output again with the same implementation can prove stability while preserving the same non-standard dialect. The ACM name and TDM order defects are examples. Round-trip tests need independent evidence: XSD for XML, standard-derived golden files for KVN, cross-implementation fixtures where licensing permits, and semantic comparisons across notations.

Those semantic comparisons must include meaningful comments and CCSDS-defined user data wherever
the relevant combination supports them. Whitespace, numeric spelling, and other presentation
details may be canonicalized.

### M4. Fuzz targets prove only non-panicking parse attempts

The three fuzz targets call parsing functions and discard results. The corpus is sizeable and
useful, but targets do not exercise generation, diagnostic determinism, round trips, schema
compliance, or resource bounds. Existing timeout artifacts also deserve triage rather than
remaining unexplained.

Add structured/property targets: successful parse must generate without panic; canonical output
must reparse equivalently; any future recovery profile must produce diagnostics while ordinary
parsing rejects the same deviation; and bounded inputs must stay within documented memory/time
limits.

### M5. Benchmarks are narrow and lack quality gates

The scale benchmarks concentrate on synthetic OEM histories. They do not expose the nullable-heavy XML path, combined detection, OCM/ACM/AEM row allocation, diagnostics, or writer streaming. There are no allocation measurements or stated regression budgets.

Build a representative benchmark corpus by format, message type, size, optional-field density, validity, and recovery path. Track throughput, latency, peak resident memory, allocations, and output bytes in CI with noise-aware thresholds.

### M6. Hand-maintained serializers and visitors are drifting

The ACM name and TDM order defects are concrete drift. Large message files, repeated visitors, separate KVN/XML ordering logic, and hundreds of serde annotations make further divergence likely.

Do not blindly generate the entire public API from XSD: the prose contains semantics that XSD cannot express. Do generate or verify mechanical facts such as XML names, sequence order, occurrence bounds, fixed version, unit enums, and fixture coverage. Keep semantic validation hand-written and reviewed. The support/conformance matrix should be the authoritative index connecting these checks to each advertised combination.

### M7. Quality polish does not match mission-critical claims

Examples include a duplicated AEM doc comment at `ccsds-ndm/src/kvn/aem.rs:108`, an always-true strict-or-lenient condition at `ccsds-ndm/src/messages/tdm.rs:81`, repeated `false || ...` expressions in OEM tests, stale implementation-history comments such as “Corrected” and “rest of logic remains similar,” and ignored formatting results throughout the writer. Strict Clippy currently reports 43 errors across all targets.

These are not the primary correctness problems, but they are warning signs in code advertised for mission-critical use. Replace historical commentary with invariant-focused documentation, remove tautologies and duplication, and require focused human review for large generated-looking changes.

### M8. Determinism and semantic preservation lack explicit gates

The project goal promises deterministic generated output and semantic round trips, but the current
review does not identify direct tests that generate the same validated model repeatedly, compare
KVN and XML meanings independently, or verify preservation of supported comments and CCSDS-defined
user data.

**Required change:** add deterministic-output tests for every public writer and semantic
round-trip tests across KVN and XML. Include supported comments and CCSDS-defined user data in the
comparison, while allowing documented presentation normalization.

## Recommended target architecture

### 1. Separate syntax recovery, canonicalization, and validation

Use a pipeline with explicit boundaries:

```text
bytes/events
  -> syntax parser + located diagnostics
  -> loss-aware source representation
  -> edition-aware canonical model
  -> complete semantic validation
  -> validated canonical message
  -> edition-specific KVN/XML writer
```

This prevents “lenient” from meaning “discard whatever did not parse” and prevents generators from seeing invalid state.

### 2. Define strictness as policy, not global state

Parsing is currently strict-only. If concrete interoperability fixtures justify deterministic
recoveries, a future API may use this shape:

```rust
pub enum ParseMode {
    Strict,
    Permissive,
}

pub struct ParseReport<T> {
    pub message: T,
    pub diagnostics: Vec<Diagnostic>,
}
```

`from_str` remains strict and returns the message directly. A future `from_str_with_mode` would
return a `ParseReport`, associating every permissive recovery with that parse operation.
`Permissive` must recover only through enumerated rules and report every recovery; it must not
suppress arbitrary validation failures. A broader options structure should be added only after
independent policies are justified by real inputs.

### 3. Make output conformance a hard invariant

- Preserve the source edition of a parsed message by default when that edition has a complete,
  edition-correct validator and serializer.
- Target the latest fully supported edition for newly constructed messages.
- Require explicit target selection when the source edition cannot be generated, and refuse an
  exact historical target unless it has complete normative and executable evidence.
- Validate before writing and fail with all relevant diagnostics.
- Run every generated XML fixture through XSD in tests.
- Maintain standard-derived KVN golden files and field-order inventories.
- Ensure `to_kvn`, `to_xml`, and streaming writers share identical validation and conformance
  behavior.
- Reject arbitrary vendor extensions outside explicitly modeled CCSDS extension mechanisms.

### 4. Build streaming first, convenience second

Implement parsing over `BufRead`/byte slices and output over `Write`. High-volume history blocks should support iterators/callbacks so applications do not need to retain millions of records. The existing `&str -> T -> String` functions should remain ergonomic wrappers.

### 5. Establish measurable performance contracts

Before optimizing individual parsers, agree on workloads and budgets. Recommended gates:

- MB/s and records/s for KVN and XML by message family;
- allocation count and bytes per record;
- peak memory as a function of input size;
- generation throughput to an in-memory buffer and a buffered file;
- compatible-mode overhead;
- maximum accepted line/document sizes and predictable failure behavior.

## Suggested implementation order

1. Fix ACM naming and TDM element order; make XSD tests mandatory and exhaustive.
2. Gate every generator behind complete validation and reject non-finite values.
3. Remove silent OCM line skipping and fix namespace/combined detection.
4. Implement the decided version-output policy: preserve supported source editions, use the latest
   supported edition for new messages, and require explicit lossless edition changes.
5. Keep parsing strict-only until real fixtures justify enumerated recovery rules and structured
   parse outcomes.
6. Add streaming reader/writer APIs and representative allocation benchmarks.
7. Replace JSON-backed XML optional parsing and unify numeric hot paths.
8. Harden primitive types, units, and absolute/relative time semantics.
9. Reduce mechanical schema drift with generated inventories and conformance tests.

## Product decisions

1. **Parse behavior — decided:** parsing is strict-only. A permissive API is deferred until real
   fixtures justify enumerated recoveries and structured diagnostics.
2. **Historical output — decided:** generation preserves a parsed message's source edition when that edition has a complete validator and serializer. Newly constructed messages target the latest fully supported edition. Other targets are explicit, and unsupported or lossy targets are refused.
3. **Unknown extensions — decided:** arbitrary vendor extensions outside CCSDS-defined extension mechanisms are unsupported. They are rejected with a located diagnostic and are never silently discarded, preserved, or regenerated. Explicitly modeled CCSDS extension mechanisms follow their own documented support-matrix entries.
4. **Support matrix — decided:** maintain an auditable message type × edition × notation matrix linking every advertised combination to normative requirements and tests. Unsupported or incomplete combinations remain unadvertised.
5. **Scale target:** what are the expected largest OEM/OCM/AEM files, record counts, and throughput/latency targets? These numbers should drive the streaming model and performance gates.
6. **Canonical fidelity — decided:** semantic equivalence is required. Meaningful comments and CCSDS-defined user data are preserved where supported; whitespace, numeric spelling, and other presentation formatting may be normalized.

## Release Bar for Reference-Status Claims

Do not claim standards-compliant generation or mission-critical suitability until all blockers are closed and CI proves:

- every advertised combination is present in an auditable support/conformance matrix with complete normative and test evidence;
- every shipped fixture generates XSD-valid XML;
- generated KVN has equivalent standard-derived golden coverage;
- every public generator rejects invalid/unrepresentable state;
- no compatible recovery silently loses data;
- historical version behavior is explicit and edition-correct;
- strict and compatible policies are deterministic and fully diagnostic;
- generated output is deterministic and KVN/XML round trips preserve supported comments, CCSDS-defined user data, and message semantics;
- streaming and materialized APIs provide the same conformance behavior;
- fuzzing covers parse and generation invariants with resource limits;
- performance and allocation budgets are measured on representative corpora;
- unsafe code is absent or justified, panics are excluded from public data paths, and failures are returned as located errors.

This Rust-core review does not establish the project goal's Python parity requirement. That
requirement must be verified separately before making a project-wide reference-quality claim.
