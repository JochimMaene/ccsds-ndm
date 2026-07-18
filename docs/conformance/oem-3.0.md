# OEM 3.0 Rust Conformance Inventory

This inventory covers the seven advertised Rust cells for CCSDS OEM 3.0: strict KVN and XML
parsing, typed-model validation, KVN and XML generation, and conversion in both notation
directions. The normative sources are CCSDS 502.0-B-3 with Editorial Corrigendum 1 and the
NDM/XML 4.0.0 schema set with OEM schema 3.0.

## Requirement map

| Area | Normative source | Implemented behavior | Executable evidence |
| --- | --- | --- | --- |
| Message identity and structure | ODM 5.1–5.2; tables 5-1 through 5-4 | One OEM root, ordered header/body/segments, one object throughout the message, and a fixed time system | `oem_strict_parsing`, `oem_parse_diagnostics`, `oem_validation` |
| KVN lexical and record structure | ODM 5.2.4–5.2.5, 7.3–7.9, A2.5.3 | Printable ASCII, 254-character lines, LF/CRLF handling, fixed keyword order, exact 7/10-field ephemeris records, exact triangular covariance rows, and normative comment placement | `oem_strict_parsing`, `oem_generation_conformance` |
| XML structure | ODM 8; `ndmxml-4.0.0-oem-3.0.xsd` and common schema | Exact root/envelope, ordered known elements, bounded nesting, no DTD or trailing document, and rejection of unknown model content | `oem_strict_parsing` |
| Time semantics | ODM 5.1.3, 5.2.3–5.2.5, 7.5.10 | Absolute OEM time tags, consistent metadata spans, ephemeris records within their total span and in nondecreasing order, and strictly increasing covariance epochs | `oem_validation` |
| Typed values | ODM 5.2 and 7.5 | Required content, finite numeric values, interpolation/degree dependency, and fixed implicit OEM units normalized across notations | OEM unit tests, `oem_validation`, `oem_conversion` |
| KVN generation | ODM 5.2, 7.3–7.9 | Deterministic ordered output, exact ODM-compatible numbers with at most 16 significant digits, complete acceleration triples, printable bounded lines, and validation before output | `oem_generation_conformance`, `oem_kvn_allocations` |
| XML generation | OEM 3.0 XSD in NDM/XML 4.0.0 | Deterministic validated XML; every shipped OEM fixture generates output accepted by the official schema | `oem_generation_conformance` |
| Conversion | ODM 5 and project semantic-preservation policy | KVN↔XML preserves the complete normalized typed model and edition; XML states with partial acceleration fail KVN conversion instead of becoming ambiguous | `oem_conversion` |
| Resource behavior | Project conformance policy | Optional exact input/output, XML-depth, and history-record limits; atomic file replacement; allocation-stable streaming KVN generation | `oem_strict_parsing`, `oem_conversion`, `oem_generation_conformance`, `oem_kvn_allocations` |
| Scale | Project performance contract | Reproducible KVN parse/generate workloads at 10–50,000 records and XML workloads at 100–10,000 records; timing remains informational | `just bench-oem` |

## Deliberate boundaries

- OEM XML permits independently optional acceleration elements. KVN has only fixed 7- or
  10-field ephemeris records, so partial acceleration remains valid XML but is rejected at the KVN
  generation boundary.
- The library does not require `INTERPOLATION_DEGREE + 1` records because the shipped normative
  examples intentionally omit intermediate records. Such a check would reject the reference
  corpus without the complete exchanged data set.
- Covariance epochs are required to be strictly increasing. They are not forced inside the
  metadata total span because the normative XML example `oem_g14.xml` places its covariance epoch
  beyond `STOP_TIME`; the library does not invent a stricter rule where the authoritative inputs
  conflict.
- This verification covers the public Rust surface only. It makes no OEM Python or CLI parity
  claim.

## Reproduction

Run `just verify-oem` for formatting/linting, the full Rust regression suite, focused conformance
tests, benchmark compilation, documentation, and packaged-crate verification. Run
`just bench-oem` separately to collect informational scaling measurements on the current host.
