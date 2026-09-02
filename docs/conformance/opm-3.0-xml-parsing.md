# OPM 3.0 XML Parsing Requirement Inventory

Capability: `OPM-3.0-XML-PARSE-RUST`

The NDM/XML 4.0.0 schema set and OPM 3.0 schema are primary for XML structure and facets;
CCSDS 502.0-B-3 with editorial corrigendum 1 controls semantics not expressed by the schema.

| Requirement area | Status | Executable evidence |
| --- | --- | --- |
| Standalone root, attributes, and complete document | Covered | `opm_strict_xml_parsing` rejects wrong/qualified roots, unknown root attributes, DTDs, trailing elements, and multiple documents. Root id/version are checked by shared validation. |
| Elements, attributes, occurrence, and sequence | Covered | OPM-specific `deny_unknown_fields` plus the small schema-sequence preflight reject unknown nested content, duplicate fixed fields, and reordered elements without a runtime XSD dependency. The preflight permits `units` only where the schema type declares it and `parameter` only on `USER_DEFINED`. As a compatibility extension, `nil`/`xsi:nil` is accepted on otherwise attribute-free optional values; attributes cannot be hidden on a nil or empty value. All shipped XML/KVN-derived fixtures parse and generated XML passes the official XSD. |
| Lexical types, units, numerics, epochs, and semantic validation | Covered | Typed deserialization plus the shared OPM validation mutation/boundary suites cover all public numeric blocks, epoch positions, unit wrappers, required strings, anomaly choice, maneuvers, and user data. |
| Optional blocks and meaning preservation | Covered | `opm_conversion` compares complete typed models across every shipped fixture in both notation directions, including comments, units, repeated maneuvers, covariance, and user-defined parameters. |
| Structured diagnostics | Covered | `opm_parse_diagnostics` fixes operation/notation/message/edition/code and semantic model paths. XML locations remain absent where `quick-xml` does not reliably expose them; the API does not invent a position. |
| Resource behavior | Covered | `opm_parse_limits` enforces an optional exact input-byte bound and a safe default XML depth of 16 with caller override. |
| Performance and fuzzing | Covered | `xml_parse_opm` plus early/late-invalid and configured-limit workloads are in the Criterion/CodSpeed-compatible corpus. The generic XML fuzz target reaches OPM from a checked-in minimal OPM seed, and the reproducible smoke command completes without a crash. Strict structure preflight is allocation-bounded; wall-clock thresholds and sustained fuzzing are optional discovery/maturity work, not deterministic pre-1.0 gates. |

The structure preflight contains only OPM root and sequence checks that serde cannot enforce. It is
not a general XSD engine and does not add a runtime schema dependency.
