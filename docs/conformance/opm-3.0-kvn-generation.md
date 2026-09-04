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
| §§3.2.1–3.2.4 OPM semantic/header/metadata/data requirements | Covered | The shared typed-model and mutation evidence linked from the XML-generation inventory applies before either notation is emitted. Exchange-context requirements—SANA originator/center values, ICD-defined frames/time systems/user parameters, and physical consistency—are explicitly caller context; a self-contained message cannot decide them and the generator does not guess. |
| §§3.2.2–3.2.4 and 7.4.8 fixed keyword/block order | Covered | All four shipped Annex G OPM fixtures regenerate the same assignment-key sequence. Optional Keplerian, spacecraft, covariance, repeated maneuver, and user-defined blocks are exercised. |
| §§7.7.1 and tables 3-1–3-3 OPM units | Covered | Annex G generation preserves whether each optional unit is present and its exact typed spelling. Covariance generation now retains units held by the model instead of silently dropping them. ODM 7.7.1 admits only the table spelling in KVN, so the uppercase GM spelling that the XML schema also permits is canonicalized to `km**3/s**2` on output instead of failing generation; XML output keeps the spelling held by the model. |
| §§7.8.3–7.8.7 comment placement and content | Covered | Annex G generation preserves comments relative to the surrounding assignment keys, not just their flattened text order. The ODM header parser assigns only comments immediately after the version to the header, preventing a metadata comment from being duplicated across blocks. |
| §§7.3.2–7.3.4 line length and character repertoire | Covered | OPM free-text and user-defined fields reject controls, non-ASCII text, and resulting lines over 254 characters before materialized or streaming output. Numeric output has a bounded canonical spelling. The remaining variable non-text fields are calendar epochs, whose stack-backed type enforces a 64-byte maximum; focused evidence fills every OPM epoch position at that boundary. Versions, keywords, and units are closed/static values. Every generated Annex G line is also checked for printable ASCII and the 254-character limit. |
| §§7.4.3–7.4.7 assignment syntax and keywords | Covered | The internal writer emits one uppercase library keyword per line with insignificant padding. Normative §7.4.4 requires uppercase keywords without blanks; OPM table 3-3 otherwise permits a variable-length user-specified suffix. Focused generation and reparsing evidence accepts letters, digits, underscores, a leading suffix digit, and printable punctuation. It rejects empty suffixes, lowercase, blanks, non-ASCII text, and `=` because §7.4.3 permits only one assignment per line. Annex F's narrower `A-Z`/`0-9`/underscore regex is treated as informative guidance, not an invented mandatory restriction. |
| §§7.5.1–7.5.11 values and epochs | Covered | Shared typed validation rejects missing required values, invalid calendar epochs, non-finite numbers, invalid choices, and covered ranges before generation. OPM KVN numeric output retains `zmij`'s allocation-free shortest representation when it fits and rounds 17-digit values to the 16-digit ODM limit. Focused evidence covers the 16/17-digit boundary, long fixed-to-scientific normalization, signed minimum subnormals, streaming parity, and isolation from XML generation. |
| Pre-write validation across public Rust generation | Covered | `Ndm::to_kvn`, typed streaming, type-erased, and file generation share the complete-model gate. Focused evidence verifies invalid KVN text produces stable diagnostics, streaming writes zero bytes, and file generation leaves an existing destination unchanged. Combined-NDM composition is outside this standalone OPM capability. |
| Deterministic output and public API contract | Covered | Existing public signature tests cover the typed, version-aware, type-erased, streaming, and file entry points. The richest Annex G fixture produces identical bytes through each entry point. |
| Diagnostics and resource behavior | Covered | The [minimal diagnostic contract](../design/opm-generation-diagnostics.md) and shared regression tests cover structured context and pre-write validation. The numeric formatter uses a stack-resident `zmij` buffer and writes directly into the destination or a reusable line buffer without a per-value `String`; `opm_kvn_allocations` enforces materialized and pre-sized streaming allocation budgets. |
| Representative performance | Covered | `just bench-opm-kvn` reproducibly measures materialized and streaming generation of the richest Annex G numeric/covariance fixture, plus materialized generation with 1, 10, 100, and 1000 repeated maneuvers. The scaling benchmark keeps parsing and model construction outside the measured loop, while `opm_kvn_allocations` enforces deterministic allocation budgets. Wall-clock results are informational under the proportionate pre-1.0 policy. |

No known KVN-generation requirement gap remains. Run the linked verification recipe when changing
the cell status.
