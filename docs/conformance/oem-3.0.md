# OEM 3.0 Core Conformance Inventory

This inventory covers the core CCSDS OEM 3.0 behavior: strict KVN and XML parsing, typed-model
validation, KVN and XML generation, and conversion in both notation directions. The normative
sources are CCSDS 502.0-B-3 and the NDM/XML 4.0.0 schema set with OEM schema 3.0. The local book
contains Editorial Corrigendum 1; the review also checked the
[published EC2 edition](https://ccsds.org/publications/allpubs/entry/3073/), whose corrections do
not change the OEM requirements. This is evidence for the implemented behavior, not a claim of
complete conformance; see the unresolved requirements below.

## Requirement map

The book and schema are the intended authority: rules come from them, not from project policy.
Where they are silent or conflict, the reading taken is recorded below, and the remaining limits
list where the implementation still falls short of them.

| Area | Normative source | Implemented behavior | Executable evidence |
| --- | --- | --- | --- |
| Message identity and structure | ODM 5.1–5.2; tables 5-1 through 5-4 | One OEM root, ordered header/body/segments, one object throughout the message (7.5.9 blank/underscore and 7.5.3 case equivalence), and a fixed time system | `oem::parsing`, `oem::diagnostics`, `oem::validation` |
| KVN lexical and record structure | ODM 5.2.4–5.2.5, 7.3–7.9, A2.5.3 | Printable ASCII, 254-character lines, every line terminated (LF/CR/CRLF/LFCR), blank lines anywhere, insignificant trailing blanks, fixed keyword order, exact 7/10-field ephemeris records, exact triangular covariance rows, an optional empty covariance section, empty optional values treated as absent, and normative comment placement | `oem::parsing`, `oem::generation`, `kvn::parser` unit tests |
| XML structure | ODM 8; NDM/XML 4.2–4.3; `ndmxml-4.0.0-oem-3.0.xsd` and common schema | Exact first-line declaration, required `xmlns:xsi` root declaration, unqualified or qualified (`urn:ccsds:schema:ndmxml`) element forms, namespace declarations and schema-location hints on any element, ordered known elements, no DTD or trailing document, and rejection of unknown model content | `oem::parsing`, `library::xml` |
| Time semantics | ODM 3.2.3.2, 5.1.3, 5.2.3–5.2.5, 7.5.10 | Calendar or ordinal time tags with at most 16 fractional digits, three-digit-day durations for MET/MRT, consistent metadata spans, nonoverlapping consecutive useable spans when both bounds are given, ephemeris and covariance records within their total span, and covariance epochs in increasing order, equal epochs allowed (an interpretation, see below) | `oem::validation` |
| Typed values | ODM 5.2, 7.5, 8.13 | Required content, single-case KVN spelling (7.5.3) of the TIME_SYSTEM and frame values the book lists, XML state and covariance values in the xsd:double lexical space including `INF`/`-INF`/`NaN` (8.13.4), finite KVN numbers, interpolation/degree dependency, and fixed implicit OEM units normalized across notations | `oem::model`, `oem::validation`, `oem::conversion` |
| KVN generation | ODM 5.2, 7.3–7.9 | Deterministic ordered output, ODM-compatible numbers rounded when necessary to at most 16 significant digits, complete acceleration triples, finite numbers only, printable bounded lines, and validation before output | `oem::generation`, `oem_kvn_allocations` |
| XML generation | OEM 3.0 XSD in NDM/XML 4.0.0; NDM/XML 4.2–4.3 | Deterministic validated XML with the required declaration and `xmlns:xsi`/`xmlns:ndm` root declarations; special values written as `INF`, `-INF` and `NaN`; every shipped OEM fixture generates output accepted by the official schema | `oem::generation`, `oem::validation` |
| Conversion | ODM 5 and project semantic-preservation policy | Corpus round trips preserve the complete normalized typed model and edition; partial acceleration, non-finite XML values and comments associated with later covariance matrices fail KVN conversion because KVN cannot represent them; trailing comment blanks do not survive KVN (7.4.7) | `oem::conversion` |
| Resource behavior | Project conformance policy | Fixed XML nesting safety limit, atomic file replacement, allocation-stable streaming KVN generation, and bounded XML scratch allocations (implicit units require none per state record, including accelerations; explicit XML attributes incur parser allocations) | `ndm::parsing`, `library::output`, `oem_kvn_allocations`, `oem_xml_allocations` |
| Scale | Project performance contract | Reproducible parse/generate workloads at 100, 10,000 and 50,000 records in KVN and at 100 and 10,000 in XML; timing remains informational | `cargo bench -p ccsds-ndm --bench kvn_benches -- oem_kvn_history_scaling` and `cargo bench -p ccsds-ndm --bench xml_benches -- oem_xml_history_scaling` |

## Book readings and remaining limits

- ODM 5.2.4.4 constrains only USEABLE_STOP_TIME against the next USEABLE_START_TIME. Total spans
  may overlap, and a pair of segments is checked only when both useable bounds are present.
- Object identity across segments compares values after 7.5.9 (underscore and blank runs) and
  7.5.3 (all-uppercase and all-lowercase spellings denote one value), matching AEM.
- 7.5.3 is a KVN rule for normative values, and the only values the book itself lists are the
  time systems of 3.2.3.2 and the reference frames of 3.2.3.3. A mixed-case spelling of one of
  those, such as `Utc` or `Eme2000`, is rejected in KVN TIME_SYSTEM, REF_FRAME and COV_REF_FRAME,
  when parsing and before generation. XML text follows xsd:string (8.13.5), which the schema does
  not restrict for these fields, so XML accepts such spellings and only their conversion to KVN
  fails. ICD-defined values such as a mission frame, CENTER_NAME (whose set is the SANA
  orbit-center registry, annex B2) and free-text fields keep their spelling.
- **Interpretation:** 5.2.5.7 orders covariance matrices "by increasing time tag". Equal tags,
  for example two frames of one navigation solution, are not explicitly permitted or excluded;
  they are accepted.
- REF_FRAME_EPOCH is interpreted in TIME_SYSTEM (7.5.11) like the other OEM epochs, so under MET
  or MRT it is a duration.
- **Limit:** MET/MRT support covers the lexical form of three-digit-day durations and their
  ordering. Their physical meaning (the mission or event epoch and its time system) comes from a
  comment or the ICD (3.2.3.2), which the library does not interpret.
- **Combined messages:** ODM 8.12.7 allows only `id` and `version` on constituent message tags,
  so a schema-location hint accepted on a standalone root is rejected on a constituent.
  **Interpretation:** namespace declarations remain allowed there, because XML Namespaces does
  not treat them as attributes; constituents also inherit the `<ndm>` root's declarations.
- ODM 8.13.1 and 8.13.4 give XML numbers the xsd:double conventions, so XML state and covariance
  values may be `INF`, `-INF` or `NaN`, spelled exactly so whether written literally, through
  character references, or in CDATA. KVN numbers (7.5.5–7.5.7) are finite, so these values fail
  KVN generation.
- 7.5.10 allows as many fractional-second digits as a fixed-point number, which 7.5.6 limits to
  16; the limit is read as applying to the fraction digits. The ADM, CDM, and RDM books state the
  same limit and every family except TDM, whose book states none, enforces it for every time tag.
- An empty value for an optional KVN keyword (7.5.1 requires values only for mandatory ones) is
  read as absent. An empty XML `INTERPOLATION` is an empty `xsd:string`, which KVN therefore
  cannot distinguish from an absent method.
- A bare `COMMENT` is read as an empty comment: 7.8.5 requires a following blank, and 7.4.7 makes
  the trailing blank insignificant. Generation writes `COMMENT `.
- XML element-only content accepts XML whitespace expressed literally, through character
  references, or through CDATA. [XSD 1.0 3.4.4, clause 2.3](https://www.w3.org/TR/xmlschema-1/#cvc-complex-type)
  checks character codes; [XML Infoset 2.6](https://www.w3.org/TR/xml-infoset/#infoitem.character)
  represents all three forms as character information items. Local `xmllint` rejects whitespace-only
  CDATA despite this rule.
- **Conflict resolution:** NDM/XML 4.3.4 says `xmlns:ndm` "must next be coded", but ODM 8.3.3
  and its example G-14 omit it. Parsing follows the ODM book and does not require it; generation
  satisfies both by writing it. The books show a
  qualified root both prefixed and unprefixed, so both are accepted.
- A leading byte-order mark is accepted as an encoding signature ahead of the required
  declaration.
- Values whose 16-digit CCSDS spelling would round beyond `f64::MAX` are rejected at the
  generation boundary rather than emitted, since the rounded text reads back as infinity.
- OEM XML permits independently optional acceleration elements. KVN has only fixed 7- or
  10-field ephemeris records, so partial acceleration remains valid XML but is rejected at the KVN
  generation boundary.
- `oem_g14.xml` extends the example's `STOP_TIME` from 21:28 to 22:28, with an inline note,
  so its total span includes its covariance epoch as table 5-3 requires. A regression reconstructs
  the original informative example and verifies rejection.
- KVN interpolation degrees use the signed 32-bit integer domain from ODM 7.5.4. XML's
  `positiveInteger` permits larger values: the typed degree supports positive `u32`, and XML
  degrees above `i32::MAX` fail KVN generation before any output. **Limit:** XML degrees above
  `u32::MAX` are schema-valid but rejected, because they are outside the typed model.
- **Limit:** `xsi:type` is rejected. XSD admits it when it names the declared type, which the
  library does not track.
- **Limit:** document type declarations are rejected. The books do not address them; accepting
  them would expose entity expansion to untrusted input.
- **Unresolved semantic requirement:** ODM 5.2.4.7 requires enough records to perform the
  declared interpolation throughout each block. The library checks the degree and its conditional
  presence, but does not establish interpolation sufficiency. A blanket `degree + 1` test would
  mishandle methods using velocity/acceleration derivatives; arbitrary ICD-defined methods also
  need external interpretation. This remains a blocker to an unqualified conformance claim,
  consistent with [AEM's recorded limitation](aem-2.0.md#normative-inventory-reconciliation).
- **Not enforced:** ODM 8.3.6 says `id` and `version` are the final root attributes, while
  XML 1.0 3.1 makes attribute order insignificant. Attribute order is not checked.
- Frame, center, originator, and time-system strings are preserved rather than checked against
  a live registry. Whether a frame needs `REF_FRAME_EPOCH` or a covariance uses a different frame
  depends on its definition or the exchange agreement. Parsing does not perform frame/time
  transformations, interpolation, or physical covariance plausibility checks.
- Python parity is covered by `test_oem.py`, the shared binding tests, and the packaged-wheel
  gate in `just package-python`; the adapter contains no independent OEM rules.

## Normative inventory reconciliation

Annex A2.5.3 lists 31 OEM implementation-conformance statement rows:

| ICS rows | Subject | Evidence and limits |
| --- | --- | --- |
| 1–7 | Header | Required version, creation date and originator; optional comments, classification and message ID; strict ordering and edition restrictions. Optional XML strings retain their literal contents. |
| 8–16, 23 | Metadata | Required identity/frame/time fields, optional frame epoch, comments, and exact block boundaries. One object and time system across segments, compared under 7.5.3 and 7.5.9 without rewriting values; single-case KVN spelling of book-listed time systems and frames. Frame-dependent conditions require the exchange agreement. |
| 17–20 | Time bounds | Ordered total and useable bounds, inclusive total spans for both histories, and nonoverlapping consecutive useable spans with a shared endpoint allowed. |
| 21–22 | Interpolation | Method and positive degree represented in Rust/Python and both notations; degree required with a method. Sufficiency remains unresolved as described above. |
| 24–25 | Ephemeris | One or more state records, time tags valid for the time system, position/velocity and optional acceleration, fixed units and exact KVN widths; XML acceleration components are independently optional. OEM does not impose AEM's strictly increasing state-epoch rule. |
| 26–31 | Covariance | Optional block, which may be empty (row 30 is optional); complete 21-element lower triangle, epochs, frame and comments; exact KVN triangular row widths, increasing epochs within the total span, and optional XML units. Python full-matrix input uses the lower triangle, documented by the API. |

## Changes

The behavior changes from the September 2026 review are listed in the
[release notes](../release-notes.rst).

## Scale and readiness limits

Parsing materializes the input and owned history: memory is linear in message size. Rust writer
APIs avoid materializing output strings, but still operate on an owned message. Python adapters
copy between mutable Python records and the Rust model; NumPy access is not zero-copy, and the
OEM methods do not release the GIL. Multi-threaded Python throughput is therefore not established
by the Rust benchmarks. Enforce application input-size limits before reading untrusted messages.

The allocation regression covers 100 versus 2,000 state records, with and without acceleration,
and an explicit-unit variant. It measures allocation growth, not a fixed peak-memory ceiling.
The benchmark groups below cover synthetic state histories, not every covariance/segment mix.
These bounds and the unresolved conformance requirements preclude an unconditional “ready at any scale” claim.
OEM remains **Available** in the [support matrix](../support-matrix.md); historical 1.0 parsing
and 2.0 support do not acquire complete edition-specific certification from this 3.0 review.

## Reproduction

### September 2026 review verification

- `just check` completed Rust/Python linting, the 904-field binding audit, generated-stub checks,
  strict mypy checks, all Rust tests (including doctests), all Python tests, and the Sphinx build
  successfully.
- Rust formatting, Python test formatting and `git diff --check` passed.
- Both OEM benchmark groups ran in release mode with 10 samples, 0.2 seconds of warmup and
  a requested one-second measurement window per workload. The shared host showed substantial
  timing variance; these runs establish working workloads, not a throughput guarantee or a
  controlled performance comparison. Criterion results are under `ccsds-ndm/target/criterion/`.
- Packaged-artifact gates were not rerun; this review changes no packaging configuration.

### Commands

Run `just verify` for the full quality checks plus packaged-artifact gates. Run `cargo bench -p ccsds-ndm --bench kvn_benches -- oem_kvn_history_scaling` and `cargo bench -p ccsds-ndm --bench xml_benches -- oem_xml_history_scaling` separately to collect informational scaling
measurements on the current host.

The `oem`, `oem_kvn_allocations`, and `oem_xml_allocations` suites carry the focused evidence,
plus `library::output` for the shared cross-family output and atomic-file guarantees.
