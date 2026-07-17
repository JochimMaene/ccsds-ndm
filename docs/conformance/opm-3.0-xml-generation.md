# OPM 3.0 XML Generation Requirement Inventory

Capability: `OPM-3.0-XML-GENERATE-RUST`

Normative inputs:

- NDM/XML schema set 4.0.0, including the OPM 3.0 schema, as the primary authority for XML
  structure, lexical types, and value facets;
- CCSDS 502.0-B-3, *Orbit Data Messages*, April 2023;
- CCSDS 502.0-B-3 EC 1, May 2023, for semantics not expressed by the XSD.

This is the initial requirement inventory for generating a standalone OPM 3.0 XML document through
the Rust API. `Covered` means the cited evidence exercises the requirement. `Partial` or `Gap`
prevents the capability from becoming `verified`.

Checks that require information outside a self-contained OPM—such as an exchange-partner ICD,
registry membership, or the physical definition of a reference frame and time system—are caller
context. They are identified below but do not count as missing self-contained generation behavior.
The library must not invent that context or reject schema-valid extension values.

Evidence links used below:

- [OPM model and generation validation](../../ccsds-ndm/src/messages/opm.rs);
- [shared OPM data types and validation](../../ccsds-ndm/src/common.rs);
- [XML serializer](../../ccsds-ndm/src/xml.rs); and
- [focused OPM 3.0 XML-generation tests](../../ccsds-ndm/tests/opm_3_xml_generation_conformance.rs);
- [Keplerian mutation and boundary tests](../../ccsds-ndm/tests/opm_keplerian_xml_generation.rs);
- [maneuver duration-unit tests](../../ccsds-ndm/tests/opm_maneuver_duration_units.rs); and
- [structured generation diagnostic tests](../../ccsds-ndm/tests/opm_generation_diagnostics.rs);
- [generation resource-limit tests](../../ccsds-ndm/tests/opm_generation_limits.rs);
- [XML allocation budgets](../../ccsds-ndm/tests/opm_xml_allocations.rs); and
- [XML benchmarks](../../ccsds-ndm/benches/xml_benches.rs).

## Inventory

| Requirement | Generated behavior | Status | Evidence or gap |
| --- | --- | --- | --- |
| 3.1.5 | One OPM describes one object. | Covered | The [OPM model](../../ccsds-ndm/src/messages/opm.rs) has exactly one segment, matching the standalone OPM XSD; `every_shipped_opm_3_fixture_generates_xsd_valid_xml` exercises both KVN- and XML-derived messages. |
| 3.2.1 | Output contains header, metadata, data, and optional comments. | Covered | The [OPM model](../../ccsds-ndm/src/messages/opm.rs) fixes this structure and `every_shipped_opm_3_fixture_generates_xsd_valid_xml` validates representative output against the official XSD. |
| 3.2.2 and table 3-1 | Root version is 3.0; header order, required fields, and optional fields follow the standard. | Covered | The [generation gate](../../ccsds-ndm/src/generation.rs) limits OPM XML output to supported editions; model validation rejects empty fields and requires a real absolute calendar or ordinal `CREATION_DATE`; fixture XSD tests cover structure and order; text mutation tests reject XML-forbidden characters. Originator registry membership is caller context. |
| 3.2.3 and table 3-2 | Metadata order and required/optional fields follow the standard. | Covered | `OpmMetadata::validate` rejects empty mandatory values; OPM epoch validation checks every present `REF_FRAME_EPOCH`; text mutation and fixture XSD tests cover lexical safety, structure, and order. Registry-backed `CENTER_NAME` semantics and whether a particular frame requires `REF_FRAME_EPOCH` need caller-provided frame/ICD context. |
| 3.2.4.1–3.2.4.4 and table 3-3 | The six logical data blocks use the defined XML elements and sequence; the state vector is complete. | Covered | The [OPM data model](../../ccsds-ndm/src/messages/opm.rs) fixes element presence and order. Table-driven public-model mutations cover every state-vector component and text-bearing logical block; focused tests below cover each optional numeric block. |
| 3.2.4 Keplerian block | The block is absent or complete and contains exactly one of true or mean anomaly. | Covered | Non-optional fields enforce block completeness. `KeplerianElements::validate` checks every public numeric value against the XSD facets and enforces the anomaly choice; [focused mutation and boundary tests](../../ccsds-ndm/tests/opm_keplerian_xml_generation.rs) exercise the rules. |
| 3.2.4 spacecraft block; 3.2.4.5–3.2.4.6 | Spacecraft parameters use the defined fields and units; zero coefficients retain their specified meaning. | Covered | The typed [spacecraft model and validation](../../ccsds-ndm/src/common.rs) constrain units; `opm_3_xml_generation_rejects_every_invalid_spacecraft_value` mutates every public numeric value. Shipped fixtures exercise the block against the XSD. Zero coefficients require no output transformation. |
| 3.2.4 covariance block; 3.2.4.10 | A present covariance is complete and emitted in lower-triangular row order. | Covered | Non-optional fields and serializer order encode the complete lower triangle; `opm_3_xml_generation_rejects_every_non_finite_covariance_component` mutates all 21 public components and the covariance fixture passes the XSD. The [frame/unit audit](opm-3.0-xml-frame-unit-audit.md) confirms the units are closed XSD enums and `COV_REF_FRAME` is intentionally unrestricted. |
| 3.2.4.7–3.2.4.9 maneuver block | Each maneuver is complete and ordered; delta mass follows the XML schema rule; a message with maneuvers includes spacecraft mass. | Covered | `OpmData::validate` requires spacecraft mass. Table-driven tests mutate every public maneuver number; [focused epoch tests](../../ccsds-ndm/tests/opm_epoch_xml_generation.rs) cover ignition calendar and numeric forms; unit tests reject the XSD-invalid duration unit. Boundary evidence confirms that the XSD's `deltamassTypeZ` permits zero, despite 3.2.4.7 saying negative. Physical time-system interpretation is caller context. |
| 3.2.4.11 | Covariance and maneuver frames remain schema-valid strings; non-standard semantics require an ICD. | Covered | The XSD intentionally leaves `COV_REF_FRAME` and `MAN_REF_FRAME` unrestricted. The [frame/unit audit](opm-3.0-xml-frame-unit-audit.md) confirms that closed enums would incorrectly reject ICD-defined values; interpretation beyond non-empty required fields is caller context. |
| 3.2.4.12 | User-defined parameters use the standard XML extension mechanism. | Covered | The [typed model](../../ccsds-ndm/src/types.rs), text mutation coverage, and fixture XSD test cover `userDefinedParameters`/`USER_DEFINED` structure and XML-safe content. The required semantic description belongs to the exchange-partner ICD and is caller context. |
| 7.5.1–7.5.11, where applicable to XML | Required values, generated numeric lexical forms, and epochs follow ODM value and time-system rules. | Covered | Typed values and exhaustive block-level numeric mutation tests cover self-contained numeric constraints. [Focused epoch tests](../../ccsds-ndm/tests/opm_epoch_xml_generation.rs) cover real calendar/ordinal ranges, fractional syntax, second `60`, and rejection of timezone offsets that the broad XSD union permits but ODM §7.5.10 forbids. Physical TIME_SYSTEM/epoch consistency requires caller context. |
| 7.8.3–7.8.7 | Comments are optional and emitted only in allowed OPM positions. | Covered | Comments belong to logical model blocks, serializer order is fixed, and generated comment-bearing fixtures pass the official XSD. |
| 8.2–8.3 | The XML declaration and standalone OPM root attributes follow the ODM/XML envelope rules. | Covered | The [XML serializer](../../ccsds-ndm/src/xml.rs) emits the exact declaration, root validation fixes `id="CCSDS_OPM_VERS"` and version `3.0`, and OPM serialization emits the exact `xmlns:xsi` declaration before the final `id` and `version` attributes. [Focused envelope tests](../../ccsds-ndm/tests/opm_xml_root_envelope.rs) cover typed, versioned, and streaming generation and validate generated output against the official XSD. |
| 8.4–8.7 and schema set 4.0.0 | XML element names, ordering, occurrence, and schema types match OPM 3.0. | Covered | `every_shipped_opm_3_fixture_generates_xsd_valid_xml` validates all five shipped fixtures through the official master schema, and CI makes missing `xmllint` a hard failure. Focused tests exercise every schema-constrained public numeric field, epoch form, required string, frame/unit choice, and XML text position before serialization. |

## Project Quality Gates for This Cell

| Gate | Status | Evidence or gap |
| --- | --- | --- |
| Deterministic output | Covered | Repeated, versioned, streaming, and generic Rust entry points produce identical bytes for the representative OPM fixture. |
| Invalid-model rejection | Covered | Public Rust XML generation rejects invalid self-contained public model states before writing: required text, XML-forbidden characters, epochs, units, and every state-vector, Keplerian, spacecraft, covariance, and maneuver number have focused mutation evidence. |
| XSD-valid generated XML | Covered | All five shipped OPM fixtures pass the official schema after generation, with adversarial coverage of the schema-constrained public fields and XML text positions. |
| Public Rust API contract | Covered | The documented typed, version-aware, streaming, type-erased, and file entry points state their validation, edition-selection, output, and failure behavior. `public_opm_xml_generation_signatures_remain_compatible` fixes their public signatures in executable evidence. The raw serde XML serializer is crate-internal, so it cannot bypass the public generation gate. |
| Stable structured diagnostics | Covered | Every safely reachable OPM missing-required-field, invalid-choice, invalid-value, and out-of-range generation failure exposes a stable code and complete model path relative to the message root. The [minimal diagnostic contract](../design/opm-generation-diagnostics.md) adds a failure-only context wrapper and borrowed view with severity, operation, notation, message kind, source/target edition, code, optional path, and explicitly non-applicable generation location/recovery fields. `opm_generation_diagnostics` proves identical context across typed, versioned, streaming, type-erased, and file entry points. Sink and file failures retain `io.error` without a model path. No success-path diagnostic context is allocated. Normative identifiers remain optional and are not invented for broad error categories. |
| Panic-free and bounded output | Covered | [Failing-writer tests](../../ccsds-ndm/tests/opm_xml_writer_failure.rs) cover multiple sink-failure boundaries and verify stable I/O diagnostics without panics. `GenerateOptions::max_output_bytes` is an optional caller resource policy with no finite default. [Limit tests](../../ccsds-ndm/tests/opm_generation_limits.rs) cover exact bounds, stable resource diagnostics, explicit larger limits, and zero-byte streaming rejection. A counting preflight is used only when the caller configures a limit; unlimited streaming retains the single serialization pass. Per-field limits are not added because the aggregate bound is sufficient for this bounded-size OPM model. |
| Representative performance | Partial | `xml_generate_opm` benchmarks the richest shipped OPM fixture through both materialized and streaming XML generation, includes the shared validation gate in each iteration, and reports throughput using generated output bytes. `opm_xml_allocations` enforces materialized, streaming, and configured-limit allocation budgets and the streaming implementation writes directly to the caller sink without retaining a document buffer. `just bench-opm-xml` reproduces the workload. Published timing baselines and noise-aware regression thresholds remain open. |
| Rust surface/release gates | Partial | The [pre-1.0 Rust release policy](../rust-release-policy.md) deliberately supports current stable Rust on Ubuntu only, records the unstable compatibility contract, and does not require byte-identical artifacts before 1.0. `just package-rust` packages, extracts, installs, and executes the CLI from the artifact; clean-checkout CI also requires the tag to match the crate version. The private security-reporting route remains open. |

## Next Small Implementation Step

Choose and document a private vulnerability-reporting route before claiming a complete Rust
security gate. Keep that repository-level decision separate from OPM wire-format logic. Do not
mark the capability `verified` until every remaining `Partial` and `Gap` above is closed.
