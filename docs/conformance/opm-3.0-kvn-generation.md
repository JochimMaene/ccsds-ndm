# OPM 3.0 KVN Generation Requirement Inventory

Capability ID: `OPM-3.0-KVN-GENERATE-RUST`

Normative authority is CCSDS 502.0-B-3 with editorial corrigendum 1. Unlike XML, KVN has no XSD;
the applicable wire requirements come from ODM sections 3.2 and 7.3–7.9. Annex G provides
non-normative examples used as representative interoperability fixtures.

`Covered` means executable evidence exercises the requirement. `Partial` means useful evidence
exists but a known part remains open. Shared OPM model semantics are linked to the existing
[OPM 3.0 XML-generation inventory](opm-3.0-xml-generation.md); the rows below add the
notation-specific KVN requirements. This inventory does not advertise the capability as conformant.

| Requirement | Status | Evidence or gap |
| --- | --- | --- |
| §§3.2.1–3.2.4 OPM semantic/header/metadata/data requirements | Partial | The shared typed-model and mutation evidence linked from the XML-generation inventory applies before either notation is emitted. Exchange-context requirements—SANA originator/center values, ICD-defined frames/time systems/user parameters, and physical consistency—remain caller context and are not claimed as self-contained validation. |
| §§3.2.2–3.2.4 and 7.4.8 fixed keyword/block order | Covered | All four shipped Annex G OPM fixtures regenerate the same assignment-key sequence. Optional Keplerian, spacecraft, covariance, repeated maneuver, and user-defined blocks are exercised. |
| §§7.7.1 and tables 3-1–3-3 OPM units | Covered | Annex G generation preserves whether each optional unit is present and its exact typed spelling. Covariance generation now retains units held by the model instead of silently dropping them. KVN preflight rejects the uppercase GM unit spelling permitted only by the XML schema. |
| §§7.8.3–7.8.7 comment placement and content | Covered | Annex G generation preserves comments relative to the surrounding assignment keys, not just their flattened text order. The ODM header parser assigns only comments immediately after the version to the header, preventing a metadata comment from being duplicated across blocks. |
| §§7.3.2–7.3.4 line length and character repertoire | Partial | OPM free-text and user-defined fields reject controls, non-ASCII text, and resulting lines over 254 characters before materialized or streaming output. Representative generated documents contain only printable ASCII and compliant line lengths. Numeric formatting can still produce an overlong line for extreme finite values and remains open below. |
| §§7.4.3–7.4.7 assignment syntax and keywords | Partial | The internal writer emits one uppercase library keyword per line with insignificant padding. User-defined keyword suffixes reject lowercase letters and blanks. Complete allowed-character evidence for user-defined suffixes remains open. |
| §§7.5.1–7.5.11 values and epochs | Partial | Shared typed validation rejects missing required values, invalid calendar epochs, non-finite numbers, invalid choices, and covered ranges before generation. Rust's shortest-round-trip float formatting may emit 17 significant digits, while §§7.5.6–7.5.7 permit at most 16; a compliant numeric formatter is still required. |
| Pre-write validation across public Rust generation | Covered | `Ndm::to_kvn`, `VersionedNdm::to_kvn_with`, typed streaming, type-erased, and file generation share the complete-model gate. Focused evidence verifies invalid KVN text produces stable diagnostics, streaming writes zero bytes, and file generation leaves an existing destination unchanged. Combined-NDM composition is outside this standalone OPM capability. |
| Deterministic output and public API contract | Covered | Existing public signature tests cover the typed, version-aware, type-erased, streaming, and file entry points. The richest Annex G fixture produces identical bytes through each entry point. |
| Diagnostics, resource behavior, and performance | Gap | Complete KVN-generation diagnostic mutations, failing-writer coverage, numeric resource bounds, and an OPM KVN generation benchmark/budget are not yet linked to this cell. |

## Next Small Implementation Step

Introduce one allocation-conscious KVN numeric formatter that emits at most 16 significant digits
without overflowing finite boundary values, then use it for OPM numeric fields. Keep XML numeric
serialization unchanged and measure the KVN generation impact before applying the formatter to
other message families.
