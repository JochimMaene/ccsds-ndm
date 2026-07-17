# Conformance and Product Policy

This document defines the target behavioral contract for `ccsds-ndm`. The support and conformance matrix, once published, is the authoritative statement of which parts of this contract a release currently satisfies.

## Product Model

The applicable official CCSDS schemas are authoritative for notation-specific wire structure and
schema constraints. The applicable CCSDS publications and corrigenda are authoritative for
semantics and requirements not expressed by those schemas. A published support and conformance
matrix maps those inputs to executable evidence in the shared Rust core. Rust, Python, and
command-line interfaces expose that same behavior without developing separate parsing, validation,
conversion, generation, or diagnostic semantics.

Core parsing and generation are deterministic and offline. Optional caller-provided integrations may supply external context such as identifier resolution or mission conventions, but results distinguish self-contained conformance from checks that could not be performed without that context.

## Operations

The project uses the following terms consistently:

- **Inspect** reads structure without claiming that the input is conformant. It may retain unsupported or nonconformant source records as uninterpreted content, but it reports their presence and never places them silently in the canonical typed model.
- **Parse** constructs an edition-aware typed model while enforcing the syntax, structure, lexical forms, units, and representation rules covered by the advertised capability.
- **Validate** evaluates all applicable self-contained semantic requirements, plus any checks enabled by explicitly supplied caller context, and returns all relevant diagnostics.
- **Convert** produces another notation or edition while preserving supported message meaning. A notation-only conversion preserves the source edition.
- **Generate** emits deterministic, edition-correct KVN or XML from a complete validated model.

Public strict parsing entry points perform parsing followed by all self-contained validation covered
by the advertised capability before returning a typed model. The operations remain separate matrix
dimensions: validation can be invoked independently after construction or mutation, and each
operation requires its own evidence.

Raw notation parser combinators, serde helpers, and KVN writers are implementation details, not
alternate public processing surfaces. Rust callers parse complete typed messages through
`Ndm::from_kvn`, `Ndm::from_xml`, or the crate-level auto-detection helpers, and generate through
`Ndm`, `VersionedNdm`, or `MessageType`, so the validation boundary cannot be skipped accidentally.

Conformance means compliance with the syntactic and semantic requirements that can be decided from the message and explicitly supplied caller context. It does not prove that mission data is factually correct, that an external identifier exists, or that an asserted physical state is true.

Canonical output is deterministic, edition-correct output that preserves supported message meaning. It is not necessarily byte-for-byte identical to the source.

## Advertised Capabilities

An advertised capability is an exact cell in the support and conformance matrix:

> message type × standard issue and corrigendum × notation and schema revision × operation × public surface

Conversion and generation cells also identify the target notation and edition. Public surfaces are Rust, Python, and the CLI. This granularity permits honest parse-only, notation-specific, historical, experimental, or surface-specific support without implying broader capabilities.

An edition is fully supported **on a public surface** when every advertised operation and notation for that edition on that surface meets the applicable quality bar. Project-wide parity is reported separately. A missing CLI or Python capability therefore does not erase a verified Rust capability, but the project may not imply cross-surface parity until it is demonstrated.

The conformance matrix is the authoritative support statement. README tables, package descriptions, examples, and source-code coverage may summarize implementation availability but must not advertise broader conformance.

## Core Promise

For every advertised capability, users can trust that:

- accepted messages are interpreted according to the exact applicable CCSDS issue, corrigendum, notation, and schema revision;
- ambiguous or invalid data is rejected rather than guessed;
- inspection, parsing, and conversion never silently lose, invent, repair, or reinterpret data;
- generation produces deterministic, validated, conformant output for the selected edition or returns precise diagnostics;
- diagnostics identify stable code, severity, field path, source location, applied recovery, and the relevant normative requirement where available;
- large, malformed, adversarial, and untrusted inputs fail safely within published resource bounds;
- public surfaces receive the same conformance decisions wherever parity is advertised;
- common workflows are simple without hiding the full-fidelity CCSDS model from advanced users; and
- representative performance is measured, published, reproducible, and regression-tested.

## Processing Profiles

Strict processing is the default. For an advertised capability, it rejects every covered violation at the appropriate parse or validation boundary. A capability with known rule gaps cannot be advertised as conformant.

Permissive processing is explicit. It applies only enumerated, deterministic recoveries for documented real-world deviations and reports every recovery. It never guesses at ambiguous data, suppresses arbitrary validation failures, or silently accepts, discards, preserves, or regenerates vendor extensions outside CCSDS-defined extension mechanisms.

Inspection is distinct from permissive parsing. It can expose unsupported source content for diagnosis without interpreting that content or claiming conformance.

## Generation and Version Policy

- Every public generation path validates the complete message for the selected edition before writing.
- Fully supported current editions can be parsed, validated, converted, and generated in every advertised notation on the relevant public surface.
- Documented historical editions may have operation-specific support, such as inspection or parsing without historical generation.
- Generation of a parsed message preserves its fully supported source edition by default.
- Newly constructed messages target the latest fully supported edition by default.
- KVN-to-XML and XML-to-KVN conversion preserve the source edition unless the caller explicitly requests an edition change.
- Upgrading or downgrading reports the version change and fails if the target cannot represent the source meaning without loss.
- Historical generation is available only through a complete, edition-correct serializer backed by normative and executable evidence.
- Publication of a new or revised CCSDS document does not imply support until the relevant matrix cells meet the quality bar.

## Semantic Preservation

Round trips preserve meaning rather than bytes. Meaningful comments and CCSDS-defined user data are preserved wherever the advertised capability supports them. Whitespace, numeric spelling, field padding, and other presentation details may be normalized.

Required fields, units, numeric constraints, ordering, and other wire rules follow the selected CCSDS edition. The library neither loosens nor invents requirements. Arbitrary unsupported content retained for inspection cannot be edited structurally or emitted as conformant output.

## Diagnostics

Diagnostics are a public interface. Human-readable wording may improve, but stable codes, severity, format, message type and edition, field path, source location, original token, recovery action, and structured CLI output follow the compatibility policy.

Strictness determines whether a diagnostic rejects an operation; it does not make relevant diagnostics disappear. A permissive result associates every recovery with the operation that produced it.

## Scope

The project covers CCSDS NDM representation and exchange: syntax, exact editions and corrigenda, typed values, units, self-contained cross-field semantics, validation, diagnostic inspection, KVN/XML conversion, canonical generation, and conformance evidence.

The Rust crate is published as `ccsds-ndm`. The Python distribution is published as `ccsds-ndm-py` and imports as `ccsds_ndm`. A separately maintained Python distribution named `ccsds-ndm` uses the same import namespace. Documentation therefore uses exact distribution names in installation commands, identifies which distribution it describes, recommends isolated environments when replacing or evaluating either implementation, and does not imply ownership, affiliation, compatibility, or safe coexistence.

The project is distributed under MPL-2.0. License and contribution terms are part of the adoption contract and do not change without a public decision and an assessment of compatibility impact.

Additional language bindings are added only when adoption evidence justifies their long-term compatibility and release cost. Until then, the CLI is the language-neutral integration surface.

## Non-Goals

- Orbit propagation, general frame transformation, conjunction analysis, attitude dynamics, or other astrodynamics computations.
- Treating arbitrary vendor extensions outside CCSDS-defined extension mechanisms as conformant.
- Automatically interpreting or regenerating unsupported vendor-extension meaning.
- Structured editing or byte-perfect regeneration of unsupported source content retained only for inspection.
- Byte-perfect reproduction of conformant source files.
- Guessing at damaged, incomplete, or ambiguous data.
- Runtime dependence on an XSD engine, network service, or external executable.
- API compatibility or package coexistence with the separately maintained `ccsds-ndm` Python distribution.
- Adding formats or language bindings solely to make the feature list appear comprehensive.
- Formal CCSDS certification or endorsement.

## Capability Quality Bar

An operation-specific capability is advertised only when every applicable criterion is satisfied:

- exact normative requirements and supported options are covered by an auditable matrix entry;
- strict processing rejects covered violations at the appropriate boundary;
- every permissive recovery is enumerated, deterministic, located, and diagnostic;
- inspection distinguishes typed conformant content, supported recoveries, and uninterpreted source content without silent loss;
- every public generator rejects invalid, lossy, or unrepresentable state for the selected edition;
- generated XML passes the exact applicable official XSD;
- generated KVN has equivalent standard-derived golden and field-order coverage;
- cross-notation conversion preserves supported semantics and the source edition unless a version change is explicit;
- malformed or adversarial input cannot cause silent data loss, panics, uncontrolled resource use, or hangs;
- streaming and materialized APIs provide the same conformance and diagnostic behavior where both are advertised;
- behavior, documentation, and tests agree on each advertised public surface;
- stable diagnostic codes, exit behavior, and structured output have compatibility tests;
- representative benchmarks demonstrate advertised scale and performance properties;
- release artifacts install and run on every advertised platform; and
- compatibility, security, migration, and reproducible-release policies are maintained for the public surface.

The matrix records capability-level evidence and surface/release evidence separately. Requirements
about message behavior, diagnostics, notation, semantic preservation, and resource safety apply to
the exact capability cell. Requirements about artifacts, platforms, compatibility, documentation,
security, migration, and reproducible releases apply to the public surface and release containing
that cell. A cell is `verified` only when both sets of applicable gates are green.

## Performance Contract

Performance claims include the corpus, hardware, software versions, commands, and statistical method required to reproduce them. The benchmark suite covers:

- latency for representative small messages;
- KVN and XML parsing and generation throughput in MB/s and records/s;
- streaming throughput and peak memory for large OEM, OCM, AEM, ACM, and TDM products;
- allocation count and allocated bytes per record where measurable;
- Python end-to-end performance, including native-boundary and object-materialization costs;
- permissive-processing and inspection overhead;
- deterministic failure behavior for configured input, line, record, and document limits; and
- fair comparisons with relevant independent implementations on identical public inputs.

The project does not claim to be the fastest implementation without reproducible evidence. Its durable promise is native performance with explicit, regression-tested resource behavior while preserving the same conformance decisions on every advertised surface.
