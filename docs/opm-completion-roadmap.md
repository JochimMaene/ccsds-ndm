# OPM 3.0 Completion Roadmap

This roadmap turns the project goal into an incremental path from the current OPM implementation to
complete, honestly advertised OPM 3.0 support. It is subordinate to the
[conformance policy](conformance-policy.md) and does not itself change any capability status. The
[support matrix](support-matrix.md) remains the authoritative statement of current support.

## Direction and Current Assessment

The project is moving in the right direction. Recent work has treated OPM generation as a tracer
bullet: normative requirements are inventoried, public generation validates before writing, XML is
checked against the official schema, KVN has standard-derived lexical and ordering evidence,
diagnostic codes and model paths are tested, and generation performance, scaling, and allocations
are measured.

This work is not excessive. Each mechanism closes a stated correctness, diagnostic, or performance
gate. The main overengineering risk now is building a universal diagnostic, recovery, streaming, or
context-validation framework before an OPM capability demonstrates that it needs one.

The implementation phases are complete. Strict KVN and XML parsing, self-contained validation,
both conversion directions, Rust/Python/CLI parity, structured diagnostics, bounded-resource
behavior, representative benchmarks, documentation, and artifact-installation evidence are now in
place. The reproducible verification recipe is green and each exact matrix cell is linked to its
evidence; ongoing work is to preserve those gates as the library changes.

Technical verification is deliberately proportionate for this pre-1.0 library. Independent use,
mature confidential vulnerability handling, and long-running wall-clock performance thresholds
remain reference-status goals. They do not block an exact cell whose normative behavior, resource
safety, public surface, and release artifact are reproducibly demonstrated.

## Normative Scope

The first complete scope is:

- standalone OPM, version 3.0;
- CCSDS 502.0-B-3 with editorial corrigendum 1;
- XML using the NDM/XML 4.0.0 schema set and OPM 3.0 schema;
- KVN using the normative requirements in CCSDS 502.0-B-3, especially sections 3.2 and 7.3–7.9;
- strict processing only unless real interoperability evidence later justifies a named permissive
  recovery;
- Rust and Python parse, validate, generate, and KVN/XML conversion operations;
- command-line validation and conversion, which exercise parsing and target generation without
  inventing a separate model-input format.

For XML, the official XSD is primary for structure, occurrence, lexical types, and facets. The ODM
book and corrigendum decide semantics not expressed by the schema. KVN has no XSD, so the book and
corrigendum are primary. Examples and other implementations are interoperability evidence, not
normative authority.

The following are deliberately outside this first scope:

- historical or future OPM editions;
- arbitrary vendor extensions outside the CCSDS user-defined mechanism;
- silent or heuristic recovery;
- orbit propagation, frame transformation, or physical plausibility analysis;
- mandatory online registry lookups;
- byte-for-byte reproduction of source formatting;
- combined-NDM envelopes, except where separately inventoried and advertised;
- claims that exchange-context values are correct without caller-supplied context.

## What “Complete” Means

Three milestones must not be conflated.

### 1. First verified capability

One exact support-matrix cell, preferably `OPM-3.0-XML-GENERATE-RUST`, satisfies every applicable
capability and Rust release gate. This proves that the evidence model works end to end.

### 2. Complete OPM 3.0 implementation

Strict OPM 3.0 parsing, validation, generation, and KVN/XML conversion are verified on Rust and
Python. CLI validation and conversion are verified as the language-neutral surface; they exercise
source parsing and target generation without implying a standalone CLI model-construction format.
Each exact cell is listed separately in the support matrix. Shared behavior comes from the Rust
core.

### 3. OPM reference-quality operation

Independent implementations and operational users exercise the advertised capabilities; releases,
security handling, governance, and long-term compatibility meet the reference-status scorecard.
This is earned after implementation completion and is not a reason to delay truthful verified
cells.

## Architectural Constraints

Apply these constraints throughout the roadmap:

1. Keep the Rust core as the only source of conformance decisions.
2. Validate complete public models before generation; never rely on callers using constructors
   correctly.
3. Reject ambiguity and unsupported meaning instead of guessing. Apply standard-defined numeric
   precision and rounding at notation boundaries.
4. Keep notation-specific rules at notation boundaries. Do not make XML obey KVN-only numeric or
   lexical restrictions.
5. Keep epochs as validated lexical values unless a demonstrated consumer needs chronological
   arithmetic. A time library must not become a parsing dependency merely for validation.
6. Add caller-context validation only behind explicit context supplied by the caller.
7. Do not add a permissive mode without named real-world deviations, deterministic recovery, and
   diagnostics.
8. Prefer bounded materialization for OPM. Add streaming parsing only if measured inputs or memory
   targets justify its complexity.
9. Use existing Criterion/CodSpeed infrastructure for reproducible timing evidence. Add hard
   thresholds only when stable runners and sufficient history make them meaningful.
10. Add abstractions only after the second concrete consumer appears or a current public contract
    requires one.

## Ordered Work Plan

Items are ordered by dependency and value. Each numbered subsection is intended to produce one or
more reviewable commits; it is not a mandate for one large change.

### Phase A — Finish the First Verified Rust Generation Cell

#### A1. Reconcile the remaining generation inventories

- Review every `Partial` and `Gap` in
  [OPM XML generation](conformance/opm-3.0-xml-generation.md) and
  [OPM KVN generation](conformance/opm-3.0-kvn-generation.md).
- Distinguish an actual implementation gap from caller-context semantics and project-wide release
  gates.
- Mark caller-context-only rows `Covered` when the library behavior is complete and the limitation
  is explicit; do not leave them `Partial` merely because no self-contained message can prove an
  external fact.
- Ensure every evidence link names an executable test, benchmark, or documented release gate.
- Keep XML and KVN requirements separate where the authorities differ.

Exit condition: neither inventory contains ambiguous wording about whether a row blocks
verification.

#### A2. Define the minimum generation diagnostic contract

The existing stable code and field path are useful but incomplete. Add the smallest public
structured view required by generation:

- severity;
- operation (`generate`);
- output notation (`kvn` or `xml`);
- message kind (`opm`);
- source and selected target edition;
- stable diagnostic code;
- optional model field path;
- optional normative requirement identifier where the mapping is unambiguous;
- source location and recovery fields represented as not applicable for public-model generation.

Design constraints:

- do not replace all error enums at once;
- a borrowed diagnostic view over the existing error is preferable to copying strings;
- success paths must not allocate for diagnostic context;
- human-readable messages remain free to improve;
- preserve `CcsdsNdmError::code()` and `field_path()` or provide a deliberate migration;
- parsing-specific original-token and source-location data may extend the view later without
  forcing generation to invent them.

Evidence:

- compatibility tests for every OPM generation failure category;
- identical structured context across materialized, streaming, type-erased, and file entry points;
- no field path for sink, file, or unsupported-target errors unless a model field caused them;
- documentation of which fields are stable before 1.0.

Exit condition: generation callers never need to parse prose or variant-match to identify the
operation, notation, edition, code, and applicable model path.

#### A3. Close bounded-output behavior

- Start with a caller-selectable total generated-document bound for materialized output and the
  existing normative KVN line bound.
- Add a per-collection or text bound only when measurement or an operational use case shows that
  the aggregate bound is insufficient.
- Keep these limits separate from CCSDS validity. Exceeding a caller's resource policy does not make
  the OPM nonconformant.
- Do not impose finite defaults that reject standard-valid messages until operational targets and
  measured resource behavior justify them.
- Decide whether limits belong in `GenerateOptions`, a reusable limits type, or documented
  platform bounds. Prefer one simple limits structure only if multiple limits are enforced.
- Check limits during preflight so streaming generation emits zero bytes when the model exceeds a
  configured bound.
- Return stable resource-limit diagnostics with a model path where applicable.
- Preserve an explicit way to choose larger limits; never silently truncate collections or text.
- Measure peak memory for the richest fixture and repeated-maneuver cases.
- Confirm memory grows linearly with output and does not retain a second output-sized buffer for
  streaming generation.

Do not add streaming parsing, arena allocation, or a custom allocator to solve OPM generation
unless measurements demonstrate a problem.

Exit condition: adversarial public models fail predictably within documented limits, and valid
large models have measured linear memory behavior.

#### A4. Establish proportionate performance evidence

- Keep representative parsing, validation, generation, and scaling workloads in the existing
  Criterion/CodSpeed-compatible benchmark suite.
- Record benchmark identifiers and commands so maintainers can reproduce the workloads.
- Enforce deterministic allocation and resource budgets where the measurement is stable.
- Treat wall-clock results as informational until a stable runner and enough history justify a
  noise-aware threshold.
- Avoid “fastest implementation” claims without a reproducible external comparison.

Exit condition: representative workloads are reproducible and deterministic allocation/resource
regressions are enforced. A wall-clock threshold is optional and must be evidence-driven.

#### A5. Close shared Rust release gates

- Decide and document the MSRV, or explicitly state that only current stable Rust is supported.
- Decide whether Ubuntu remains the only advertised Rust platform or add tested platforms.
- Document the currently supported security-reporting route and its limitations. A private route
  is reference-status work until the project can sustain confidential response handling.
- Install and exercise the packaged crate from the produced artifact, not only its working tree.
- Decide whether byte-reproducible artifacts are required now; if not, document the remaining gap.
- Verify tag, crate version, documentation version, and support-matrix claims agree.
- Ensure generated package contents include licenses, required documentation, and package metadata
  but omit development artifacts. Include XSD files only if a packaged conformance-test feature
  genuinely consumes them; runtime schema dependence remains a non-goal.

Exit condition: every proportionate Rust release gate referenced by the chosen generation cell is
green for the explicitly tested platform scope.

#### A6. Verify generation cells one at a time

Recommended order:

1. `OPM-3.0-XML-GENERATE-RUST`, because the official XSD supplies the strongest independent wire
   oracle.
2. `OPM-3.0-KVN-GENERATE-RUST`, reusing shared model, diagnostic, resource, performance, and release
   evidence while retaining KVN-specific requirements.

For each transition:

- run the exact conformance recipe from a clean checkout;
- confirm every inventory row is `Covered` or explicitly not applicable;
- verify all support-matrix evidence links;
- cross-check the normative mapping against the official book, corrigendum, and XSD; independent
  review is encouraged when available but is not a technical verification prerequisite;
- change only that exact cell to `verified`;
- avoid wording that implies parsing, conversion, Python, CLI, or another edition is verified.

### Phase B — Build Strict Rust Parser Evidence

Create separate cells and requirement inventories for:

- `OPM-3.0-XML-PARSE-RUST`;
- `OPM-3.0-KVN-PARSE-RUST`.

#### B1. Inventory accepted syntax

- Map every OPM element, attribute, KVN keyword, block, occurrence, ordering, unit, numeric token,
  epoch, comment position, and user-defined parameter.
- Use the XSD as the XML structural authority and the ODM book as the KVN authority.
- Record where XML schema permissiveness is narrowed by book semantics.
- Record explicitly unrestricted XSD strings so validation does not invent closed enums.
- Separate lexical validity from self-contained semantic validation.

#### B2. Verify valid input coverage

- Parse all shipped OPM KVN and XML fixtures.
- Add minimal, maximal-optional, and boundary fixtures derived from normative requirements.
- Cover calendar and ordinal epochs, fractional seconds, leap-second lexical form, and every OPM
  epoch position.
- Cover optional units present and absent where permitted.
- Cover all six logical data blocks, multiple maneuvers, comments at every allowed location, and
  multiple user-defined parameters.
- Confirm parsed values preserve units, comments, user-defined names/values, signed zero where
  meaningful, and exact epoch spelling.

#### B3. Verify invalid and ambiguous input rejection

XML:

- wrong root, `id`, or version;
- unknown elements and attributes;
- duplicates, missing required elements, and wrong order;
- invalid schema facets, units, numeric tokens, and epoch forms;
- namespace and envelope errors;
- trailing content and multiple documents;
- XML constructs the public policy does not support.

KVN:

- unknown or duplicate fixed keywords;
- missing, reordered, or repeated blocks where disallowed;
- invalid assignment syntax;
- non-printable characters and lines over 254 characters;
- invalid comments and comment placement;
- invalid units, numeric spellings, and epochs;
- malformed or ambiguous user-defined keywords;
- trailing tokens and incomplete final records;
- LF and CRLF handling as explicitly supported.

For both notations:

- reject rather than silently skip malformed rows or unknown content;
- never return a partially interpreted conformant model after an unreported error;
- add regression tests for every discovered silent-loss path.

#### B4. Add located parse diagnostics

Extend the shared diagnostic view only with parsing needs that now have a concrete consumer:

- input notation;
- byte offset;
- line and column for KVN/text XML input where reliably available;
- XML path or KVN keyword/block context;
- bounded original-token excerpt;
- expected construct;
- no recovery action in strict mode.

Avoid storing the complete input or unbounded invalid tokens in errors. Confirm malformed large
inputs cannot create oversized diagnostic allocations.

#### B5. Add parser resource limits and fuzzing

- Configure aggregate input bytes, XML depth, the normative KVN line bound, and diagnostic excerpt
  length.
- Add element or collection-specific limits only if aggregate limits fail a measured adversarial or
  operational case.
- Fail with stable resource diagnostics, not panics or hangs.
- Add OPM-focused KVN and XML fuzz seeds from all normative fixtures and boundary cases.
- Assert that parsing arbitrary bytes never panics and never silently accepts trailing content.
- Add adversarial tests for deeply nested XML, extremely long tokens, repeated fields, and large
  collections.
- Keep checked-in seeds and a reproducible smoke command. Sustained fuzzing is useful discovery
  work, not a deterministic release gate.
- Benchmark small valid input, richest valid input, invalid early/late input, and configured-limit
  rejection.

Exit condition for Phase B: parser mechanics have complete requirement inventories, located
diagnostics, bounded failure behavior, semantic-preservation tests, and performance evidence. The
parse cells remain unverified until the shared validation work in Phase C is complete and the
combined parse-plus-validation behavior is rerun.

### Phase C — Complete OPM Validation

Create an explicit `OPM-3.0-VALIDATE-RUST` cell with notation `N/A` rather than treating
notation-neutral model validation as an incidental parser or writer detail. XML/KVN lexical checks
remain at parse, generate, and conversion boundaries.

- Inventory every self-contained rule from the XSD, ODM book, and corrigendum.
- Keep required structure unrepresentable where practical, but validate all publicly mutable
  fields.
- Exhaustively cover:
  - root identity and edition;
  - header and metadata required values;
  - XML-safe and KVN-safe text as appropriate to the requested operation;
  - all epoch positions;
  - state-vector values and units;
  - Keplerian completeness, anomaly choice, ranges, and units;
  - spacecraft ranges and units;
  - covariance completeness, finite values, frame, and units;
  - maneuver completeness, mass requirement, ranges, units, and repetition;
  - user-defined parameter syntax and content.
- Decide whether validation returns all independent errors or fails fast. The project policy
  currently promises all relevant validation diagnostics, so make aggregation behavior explicit
  and bounded.
- Ensure validation itself is notation-neutral; apply XML/KVN lexical output checks only when a
  notation is selected.
- Define an optional caller-context interface only for demonstrated needs such as allowed
  originators, centers, frames, time systems, or ICD-defined user parameters.
- Never perform network registry lookup during validation.
- Add stable ordering for multiple diagnostics.
- Measure valid and invalid validation cost for small and maneuver-heavy models.
- After validation is complete, rerun the Phase B parser evidence and verify the exact XML and KVN
  parse cells. Strict public parsing means syntax parsing followed by all covered self-contained
  validation.

Exit condition: every self-contained OPM rule has an executable mutation or boundary test, while
external-context rules are clearly separated and never guessed; strict XML and KVN parse cells can
now be evaluated for verification.

### Phase D — Complete Rust KVN/XML Conversion

Create separate directional cells:

- `OPM-3.0-KVN-TO-XML-RUST`;
- `OPM-3.0-XML-TO-KVN-RUST`.

#### D1. Define semantic equivalence

- Build a comparison helper used by tests, not necessarily exposed publicly.
- Compare typed values, units, comments by logical position, epochs, optional blocks, maneuvers,
  covariance, and user-defined data.
- Ignore permitted presentation differences such as whitespace, padding, XML attribute order where
  semantically irrelevant, and canonical numeric spelling.
- Preserve source edition by default.

#### D2. Cover notation asymmetries

- XML values that require more than 16 significant digits are rounded to a conforming KVN
  representation.
- KVN-only and XML-only unit spellings must be normalized only where the standards define the same
  unit.
- Preserve optional-unit presence where the target notation permits it.
- Reject a target notation that cannot represent the source meaning after applying its
  standard-defined numeric precision.
- Preserve comments at equivalent logical positions.
- Preserve CCSDS user-defined parameters without accepting arbitrary unknown extensions.
- Ensure XML output always passes the official OPM schema.
- Ensure KVN output reparses to a semantically equivalent model.

#### D3. Exercise all public conversion paths

- typed string-to-string conversion;
- file-to-file conversion with atomic destination behavior;
- explicit target edition selection;
- type-erased dispatch;
- sink failure and unsupported-target diagnostics.

Exit condition: both directions have representative and boundary round trips with explicit
information-loss rejection.

### Phase E — Finish the Public Rust OPM API

- Review whether public construction is understandable without reading generated builder internals.
- Provide documented examples for minimal state-vector-only OPM, Keplerian data, covariance,
  spacecraft data, maneuvers, and user-defined parameters.
- Make source-preserving generation the obvious default.
- Keep target-edition changes explicit.
- Ensure typed, type-erased, materialized, streaming, and file APIs agree on validation and
  diagnostics.
- Decide and document whether public fields remain mutable before 1.0. If they do, generation and
  validation must continue to defend every invariant.
- Audit `Clone`, `PartialEq`, builders, unit wrappers, epoch wrappers, and error types for accidental
  API inconsistencies.
- Remove obsolete duplicate APIs rather than maintaining aliases during pre-1.0 development.
- Add compile-time signature tests only for APIs intentionally treated as stable.
- Publish migration notes for breaking improvements.
- Ensure rustdoc links every OPM operation to its exact supported edition and notation.

Exit condition: common OPM workflows are short and safe, advanced users retain the full typed model,
and there is one obvious API for each operation.

### Phase F — Reach Python Parity

Create exact Python cells only after the corresponding Rust behavior is verified.

- Audit every Rust OPM field, enum, unit, optional value, repeated maneuver, covariance component,
  and user-defined parameter against the Python wrapper.
- Run `just audit`, `just sync-docs`, and `just stubs` after binding changes.
- Expose strict `from_kvn`, `from_xml`, `from_file`, validation, KVN/XML generation, conversion, and
  file APIs with behavior delegated to Rust.
- Map structured Rust diagnostics to one documented Python exception hierarchy with accessible
  code, severity, operation, notation, edition, path, and source location.
- Do not reimplement validation or conversion rules in Python.
- Test invalid models, invalid input, unsupported editions, and I/O errors through Python.
- Test wheel installation in a clean environment before importing.
- Verify stubs match runtime signatures and optionality.
- Add Python examples matching the Rust examples.
- Benchmark parsing, validation, generation, conversion, and object materialization across the
  native boundary.
- Publish Python compatibility and deprecation rules separately from Rust where necessary.

Exit condition: advertised Python OPM operations make the same decisions and expose equivalent
diagnostic information as Rust.

### Phase G — Add the Minimal OPM CLI Surface

If no CLI exists, build only the commands required for the first complete OPM scope:

- `validate`;
- `convert`.

Parsing is exercised by both commands, and target generation is exercised by `convert`. Do not
invent JSON or another model-input format merely to add a standalone `generate` command.

CLI requirements:

- accept files and standard input;
- require or reliably detect notation without guessing ambiguous content;
- allow explicit OPM 3.0 and target notation selection;
- use the Rust core for every decision;
- produce concise human diagnostics by default;
- provide stable machine-readable JSON diagnostics;
- define stable exit codes for success, invalid input/model, unsupported operation/edition, resource
  limit, and I/O failure;
- never mix generated document bytes with diagnostics on standard output;
- protect existing destination files when validation fails;
- document resource-limit flags and defaults;
- add shell-level tests for exit codes, stdout/stderr separation, piping, and atomic files;
- package and install the CLI in CI.

Do not build an interactive editor, schema browser, or plugin system as part of OPM completion.

Exit condition: validation and conversion can be used safely in shell pipelines and CI without
parsing prose.

### Phase H — Documentation, Interoperability, and Release Completion

- Add an OPM guide covering parsing, validation, generation, conversion, diagnostics, limits, and
  performance expectations on each advertised surface.
- State the exact distribution names and installation commands.
- Publish a table linking every OPM capability cell to its normative inventory and executable
  recipe.
- Add end-to-end examples using only public APIs and shipped fixtures.
- Test all documentation snippets.
- Validate generated XML with the official XSD in CI.
- Reparse generated KVN and compare semantics.
- Exchange representative messages with at least one independent implementation where possible.
- Record differences as interoperability findings, never as reasons to override the standard.
- Add release notes identifying newly verified cells and remaining unsupported cells.
- Downgrade a cell immediately if a regression invalidates its evidence.

Operational adoption, governance, succession, and long-term release history remain
reference-status work after the implementation is complete.

## Suggested Incremental Milestones

These milestones keep changes reviewable:

1. Reconcile OPM generation inventory blockers without code changes.
2. Approve the minimal generation diagnostic-context API.
3. Implement diagnostic context for XML generation and its tests.
4. Reuse it for KVN generation without adding KVN-specific abstractions.
5. Add generation resource limits and adversarial tests.
6. Establish reproducible benchmark workloads and deterministic allocation/resource budgets.
7. Document the exact Rust release and security-reporting scope.
8. Verify XML generation.
9. Verify KVN generation.
10. Inventory XML parsing and add strict malformed-input evidence.
11. Inventory KVN parsing and eliminate silent acceptance/loss.
12. Verify shared OPM validation, then verify both strict parse cells.
13. Verify KVN-to-XML conversion.
14. Verify XML-to-KVN conversion.
15. Consolidate and document the Rust OPM API.
16. Reach and verify Python parity.
17. Add and verify CLI validation and conversion.
18. Complete guides, packaging, interoperability evidence, and exact matrix claims.

At each milestone:

- inspect the normative source before changing behavior;
- add the narrowest failing test first;
- implement the smallest sufficient change;
- run focused tests, then relevant full gates;
- benchmark hot-path changes;
- cross-check normative mapping and unintended API expansion against the official sources;
- update inventories and the support matrix in the same change;
- stop for review at a coherent commit boundary.

## Completion Checklist

OPM 3.0 implementation is complete only when all applicable items below are true.

### Normative and model

- [x] Exact standard, corrigendum, schema set, notation, operation, and surface are recorded.
- [x] Every self-contained OPM requirement has executable evidence.
- [x] Caller-context requirements are separated and documented.
- [x] No invented restrictions reject XSD/book-permitted values.
- [x] No arbitrary unknown extension is accepted or regenerated as conformant.

### Parsing

- [x] Strict XML parsing is verified.
- [x] Strict KVN parsing is verified.
- [x] Unknown, duplicate, reordered, malformed, and trailing content behavior is explicit.
- [x] Valid optional blocks, units, comments, epochs, maneuvers, and user-defined data are preserved.
- [x] Parse diagnostics are located, bounded, and stable.
- [x] Parser limits and adversarial tests prevent uncontrolled resource use.

### Validation

- [x] Publicly constructible invalid states are rejected.
- [x] All relevant diagnostics are returned in stable order.
- [x] Notation-neutral and notation-specific validation are separated.
- [x] Optional caller-context validation is explicit and offline.

### Generation

- [x] XML generation is verified against the official XSD.
- [x] KVN generation is verified against book-derived syntax, order, units, and lexical rules.
- [x] Every public writer validates before emitting bytes.
- [x] Materialized, streaming, type-erased, and file outputs agree.
- [x] Output failure behavior is panic-free and documented.
- [x] Resource and stable allocation budgets are enforced; timing workloads are reproducible.

### Conversion

- [x] KVN-to-XML conversion is verified.
- [x] XML-to-KVN conversion is verified.
- [x] Semantic equivalence is tested across every logical block.
- [x] Unrepresentable target values fail; finite numeric values use the target notation's
  standard-defined precision.
- [x] Edition changes are explicit.

### Public surfaces

- [x] Rust OPM APIs are coherent, documented, and packaged.
- [x] Python bindings have field, behavior, diagnostic, stub, and installation parity.
- [x] CLI validation and conversion have stable output and exit contracts; no standalone model
  generation format is implied.
- [x] No surface contains independent conformance logic.

### Operational quality

- [x] Fuzzing and malformed-input corpora cover both notations.
- [x] Representative benchmark workloads and deterministic allocation/resource budgets protect
  representative paths.
- [x] Supported toolchains and platforms are explicit.
- [x] Security reporting and release procedures are documented.
- [x] Examples and documentation are tested.
- [x] Every advertised cell is linked to reproducible technical evidence; independent review is
  tracked separately as reference-status maturity.

## Immediate Next Step

Preserve `just verify-opm` as the release evidence for these cells. Future OPM work should be driven
by a concrete interoperability finding, regression, or operational use case; do not add general
recovery, streaming-parser, security-process, or performance-threshold machinery speculatively.
