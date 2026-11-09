# CDM 1.0 conformance

This inventory records maintainer evidence for standalone CDM 1.0. The
[support matrix](../support-matrix.md) is the user-facing statement of current support.

## Scope and authorities

- Message edition: CDM 1.0.
- Semantic and KVN authority: CCSDS 508.0-B-1 with editorial corrigenda 1 and 2 and
  technical corrigenda 1 and 2 (`data/cdm_508x0b1e2c2.pdf`), principally sections 3 and 6 and annex D.
- XML authority: NDM/XML schema set 4.0.0 and CDM schema 1.0.
- Profile: tolerant standalone parsing, self-contained validation, deterministic KVN/XML generation,
  and loss-rejecting notation conversion through the shared Rust generation gate.

## Executable evidence

The `cdm` suite establishes:

| Concern | Evidence |
| --- | --- |
| KVN lexical and logical structure | The family-local scanner rejects non-ASCII values outside COMMENT payloads and forbidden control input, malformed assignments, unknown and duplicate keywords, fixed-order violations, nonstandard marked blocks, misplaced comments, extra segments, and trailing content. It models the header, relative data, two metadata sections, and each flattened OD, additional-parameter, state-vector, and covariance block in CDM order. |
| XML structure | The shared XML sequence engine is registered for the complete CDM root, header, body, relative-state vector, two segments, metadata, data, OD, additional-parameter, state-vector, and covariance hierarchy. It rejects unknown, duplicate, reordered, and non-schema children and attributes, subject to the explicit missing-value producer compatibility below. |
| Delimiter-free COMMENT decision | A leading KVN comment run before the first data keyword is normatively retained on the outer data block; no textual delimiter exists from which to infer a split. XML nested comments are merged canonically into that leading run when needed; see KVN comment merge below. Empty optional blocks do not obstruct conversion. |
| Typed preservation and generation | All three shipped KVN fixtures retain their complete typed model through deterministic KVN and XML generation. The shipped XML fixture retains its model through XML generation. Every generated XML document validates against the official 4.0.0 master schema. Numeric domains have direct validation tests in addition to the [shared XSD oracle](../design/validation-contract.md#xsd-oracle-policy). |
| KVN fixed-point preflight | CDM's shape is a fixed, bounded set of scalar records. A private generation preflight writes and reparses that bounded representation before caller-visible output, rejecting lexical violations or nonnumeric model changes outside the documented comment merge and trailing-blank trimming. Numeric rounding is checked separately under the policy below. Streaming rejection is proven to write zero bytes. |
| KVN numeric precision | Sections 6.3.2.2 and 6.3.2.3(b) limit ordinary KVN numbers to 16 decimal digits. All numeric fields use the shared formatter, so computed values such as `0.30000000000000004` round to `0.3`. The private exact-model preflight protects nonnumeric content independently of this permitted rounding; the actual rounded model must still validate before output. XML retains the full double. `cdm_kvn_rounds_computed_numbers_while_xml_retains_the_value` covers scalar/unit-bearing fields, both output forms, generic dispatch, and schema-valid XML preservation. |
| Shared surfaces | `library::parsing` and `library::output`, Python API tests, and the shared generation plumbing exercise parsing, generation, structured diagnostics, and Rust-core delegation. No separate CDM semantics are implemented in an adapter. |

## Normative inventory reconciliation

Annex A, section A2.1.5 contains 68 CDM implementation-conformance statement rows. They map to the
wire/model evidence in these groups:

| ICS rows | Subject | Reconciled implementation evidence |
| --- | --- | --- |
| 1–7 | Header | Strict version-first parsing, common header required fields, comment placement, ordering, and duplicate rejection. |
| 8–19 | Relative metadata/data | Typed relative fields and relative state vector, required-field/unit handling, exact XML order, and fixed KVN order. `RelativeMetadataData::validate` requires finite, non-negative `MISS_DISTANCE` and `RELATIVE_SPEED` because table 3-2 defines them as norms; all six relative state-vector components and `SCREEN_VOLUME_X/Y/Z` must be finite, and `COLLISION_PROBABILITY` is bounded to `[0, 1]`. `cdm_distance_and_speed_norms_require_nonnegative_scalars` covers negative input in both notations, edited models, zero boundaries, and zero-byte output refusal; Python checks the same scalar constraints. |
| 20–39 | Object metadata, repeated for OBJECT1 and OBJECT2 | Exactly two segments and one of each object designator, typed identity/orbit/covariance-method fields, required values, units, and ordering. |
| 40–58 | Object data, OD, and additional parameters | Complete flattened KVN ordering and nested XML structure, required and optional typed fields, units, and the documented delimiter-free comment decision. The shared `OdParameters::validate` and the additional-parameter validators are routed from `CdmData::validate` for both segments, covering counts, spans, percentages, masses, areas, and coefficients; signed thrust acceleration remains permitted while required to be finite. |
| 59–62 | State vector | All six required components, fixed units, ordering, and generated-schema validation. |
| 63–68 | Covariance matrix | Required lower-triangular RTN terms, optional drag/SRP/thrust extensions, fixed units, ordering, and generated-schema validation. |

The parser enforces the complete structural inventory and the self-contained
two-object/cardinality and required-field rules represented by the typed model. `ORIGINATOR` and
`CATALOG_NAME` values are not checked against the SANA registries; only the 5.2.9 character set is
enforced (see Object identity below). Object identity assignments and the choice of collision
probability method are likewise not checked against any external source. `OBJECT_TYPE` is a closed
list whose unknown values are rejected.

## Physical consistency boundary

Normative table 3-2 defines miss distance and relative speed as the norms of the corresponding
relative vectors; informative annex E repeats those meanings. Their scalar values must be finite
and non-negative even though the XSD's `lengthTypeUO` and `dvType` extend unrestricted
`xsd:double`. The library does not additionally compare them to the supplied vector norms.
The published §3.6.3 KVN example retained as `cdm_363.kvn` gives `MISS_DISTANCE = 715 [m]` with
relative position `(27.4, -70.2, 711.8) [m]`, whose norm is approximately `715.777926 m`.
Strict equality would reject this book example; the book specifies no comparison tolerance.
The library preserves the supplied values rather than inventing a tolerance or recomputing one
field from another. This is an explicit limit of validation of the book-defined relationship.

The library also does not compare screening entry/exit or start/stop times or test covariance
matrices for positive-semidefiniteness. Screening sizes and covariance entries use finite-double
domains without additional physical sign checks. It bounds collision probability to `[0, 1]`
but does not recompute it or verify the selected probability method. Successful validation
therefore does not establish these physical relationships.

## Public-surface and artifact evidence

`bindings/python/tests/test_cdm.py`, `test_api_consistency.py`, and
`test_parse_and_generation_options.py` cover Python construction, nested setters, KVN/XML parsing
and generation, generic identity, and shared limits. `package-python` and `package-rust` are the
shared built-artifact gates.

## Object identity and ordering

CDM table 3-3 requires both objects to use the same `REF_FRAME`; section 4.3.7.2 orders
`OBJECT1` before `OBJECT2`. Root validation checks both conditions, not merely that two different
object tags exist. Section 5.2.9 restricts `MESSAGE_ID`, `OBJECT_DESIGNATOR`,
`CATALOG_NAME`, and `INTERNATIONAL_DESIGNATOR` to ASCII letters/digits, `_`, and `-`; other
free-text fields retain their own domains. `cdm::model` mutates each identifier and the second
object's frame and order, checking the root and generation gates. Python construction requires
explicit covariance provenance and maneuverability rather than fabricating either required fact.

## Verification outcome

CDM 1.0 meets the common [verification criteria](README.md#verification-criteria) for
self-contained format and model validation on Rust and Python. The requirement inventory,
family tests, shared public-route tests, and packaged tests are complementary evidence;
none alone establishes conformance or exhaustive mutation coverage.

Shared Rust/Python route coverage, including `test_conformance_matrix.py`, and installed
artifact checks are indexed once in the [common evidence table](README.md#shared-evidence).

## Operational missing values and complete relative vectors

Operational CDM XML sometimes encodes unavailable optional scalar values as empty elements
or with the non-XSD attribute `nil="true"` (also `nil="1"`). For known optional numeric,
epoch, enumeration, and text leaves, an empty element or one whose decoded text contains only XML
whitespace (space, tab, CR, LF) means absent, with or without the nil marker. Empty/whitespace
CDATA and whitespace character references follow the same rule; XML comments and processing
instructions do not supply a scalar value. This applies equally to unqualified and namespace-qualified CCSDS elements;
foreign, unbound, and mixed namespaces are rejected before a missing leaf can be removed.
This is a deliberate producer-compatibility exception to the XSD, not a claim that those
original documents validate against it. Generated
XML omits absent leaves and uses the normal schema form.

Only the unqualified `nil` producer attribute is accepted; `nil="false"`, nil on required
values, nil with content, unknown attributes, and unknown/nested elements remain errors.
Supplied units are checked against the field's fixed unit before an unavailable leaf is removed;
empty, nil, and N/A payloads cannot hide incompatible units. Normalization preserves byte positions
for subsequent diagnostics. Unavailable leaves are removed before element order and cardinality
are checked, so an empty optional leaf next to a populated one (for example an empty
`<RELATIVE_SPEED/>` before a populated `RELATIVE_SPEED`, or before `MISS_DISTANCE`) is not a
duplicate or ordering error; two populated occurrences still are. Empty, comment-free optional OD and additional-parameter blocks
normalize to absent. An optional container containing only decoded `N/A` is likewise unavailable;
this compatibility does not admit `nil` attributes on containers. Comment-only blocks retain their text
through the canonical KVN comment merge below.

A relative-state vector is absent or contains all six position and velocity components.
Although the book table independently marks the components optional, the XSD's optional
`relativeStateVector` group requires all six once present. The model uses that complete group in
both notations: any incomplete subset, including velocity-only content or an empty XML vector,
is rejected rather than filling missing components. `<relativeStateVector>N/A</…>` is the 1.4.3.1
explicit not-available marker and means absent; an empty vector carries no such marker and is an XSD
violation (all six children are required) with no known producer, so it stays an error.
Missing required metadata and state/covariance
values are errors; the reader does not fabricate values from defaults.

All four book line endings (LF, CR, CRLF and LFCR) are accepted with identical models and
byte-position-preserving normalization.

## Categorical aliases and OD count profile

XML accepts the uppercase and lowercase spellings explicitly enumerated by the bundled
CDM XSD for `OBJECT`, screening volume frame/shape, `REF_FRAME`, `COVARIANCE_METHOD`,
`OBJECT_TYPE`, `MANEUVERABLE`, and the YES/NO fields `SOLAR_RAD_PRESSURE`, `EARTH_TIDES`, and
`INTRACK_THRUST`; generation uses canonical uppercase. Arbitrary mixed case is not an XML alias.
`cdm::parsing::xml_accepts_schema_listed_lowercase_categories` checks full-message preservation.

Shared `OBS_AVAILABLE`, `OBS_USED`, `TRACKS_AVAILABLE`, and `TRACKS_USED` use positive
`u32` in both notations, matching the XSD `positiveInteger` domain within the
[integer storage limit](../design/validation-contract.md#integer-storage-limits).
The book's table calls these integer counts without separately requiring positivity.
This is a deliberate shared CDM/RDM profile choice: a supplied count must also fit XML;
zero is rejected even in KVN/model form, rather than being silently omitted on conversion.
This follows the shared policy of adopting XSD facets in all notations to keep those
value domains compatible with XML. The other XSD facets applied in KVN, XML, and edited models
alike are: negative `RECOMMENDED_OD_SPAN`/`ACTUAL_OD_SPAN` (`dayIntervalTypeUO`) and
`WEIGHTED_RMS` (`nonNegativeDouble`) are rejected; `RESIDUALS_ACCEPTED` is bounded to
`[0, 100]`; negative `AREA_PC`, `AREA_DRG`, `AREA_SRP`, `MASS`, `CD_AREA_OVER_MASS`,
`CR_AREA_OVER_MASS`, and `SEDR` are rejected (`THRUST_ACCELERATION` stays signed).

## KVN comment merge

KVN cannot distinguish outer data comments from comments before its first nested data
keyword. Generation combines those runs in XML order. Comments from an optional block
without values move to the next present block; leading runs become outer data comments.
If no OD or additional-parameter values precede the state vector, that vector's leading
comments join the outer data run too. Later comments keep their association with the following
block. Empty optional blocks
normalize to absent. This preserves comment text and order, and repeating conversion is
stable; XML-to-XML retains its original associations. Space-Track layouts with nil leaves
and nested OD comments are covered by `xml_nested_comments_merge_canonically_into_kvn`.

Namespace-qualified XML introduced by Cor.2 is supported. All epochs use the calendar
branch; the shared [validation contract](../design/validation-contract.md) records the
book-over-XSD policy for empty mandatory values and epoch lexical forms.
`INTERNATIONAL_DESIGNATOR` input enforces only the 5.2.9 character repertoire, not the table 3-3
layout, because the CARA Alfano test-case CDMs use values such as `1001`/`1002`; those producer
identifiers stay readable rather than being rewritten to UNKNOWN. Generation (KVN and XML) is
strict: it refuses any value other than `YYYY-NNNP{PP}` (four digits, `-`, three digits, one to
three capital letters) or `UNKNOWN`, so such a message must be edited before output. The external
COSPAR assignment itself is not checked.
`generation::generation_enforces_the_international_designator_layout` covers this.
`TIME_LASTOB_START`/`TIME_LASTOB_END` are not compared. Covariance is optional only
during incremental construction: validation and generation require it.

KVN reading tolerates lines longer than the book's 254-character limit where applicable;
writing enforces that limit, so an accepted overlong record can require shortening before
regeneration. Shared encoding, namespace, epoch, `N/A`, whitespace and categorical rules
are specified in the [validation contract](../design/validation-contract.md).

ORIGINATOR accepts embedded blanks, including `ESA SST`, following table 3-1 Cor.1.
That example conflicts with 5.2.9's narrower repertoire; XSD is xsd:string, so input follows
the lenient reading. Other identifier repertoires remain unchanged.
Optional numeric NaN/INF leaves remain invalid; they do not denote unavailable values.
DOY and a trailing Z are accepted and retained, although 6.3.2.6 says Z should be omitted.
Fractional seconds keep the shared 16-digit cap. Closed categorical aliases include
OBJECT_TYPE and YES/NO values and read lowercase in KVN (6.2.3.3); free text retains
internal blank runs despite 6.3.2.5's text equivalence rule, avoiding changes to names.
Any incomplete subset of relative-state components is rejected.

Python state arrays carry the existing CDM field units: the six state-vector entries are
kilometres and km/s, while the relative state vector (miss geometry) is in metres and m/s.
No unit conversion occurs.

Python covariance arrays must be square, symmetric, and 6x6 through 9x9. The added
Drag/SRP/Thrust rows must be complete (5.2.8); from_numpy and the convenience setter use
one checked constructor. All entries carry the existing CDM covariance units; no unit
conversion or implicit averaging of asymmetric entries occurs.

Case rules are field-specific: KVN MANEUVERABLE accepts YES/NO/N/A and their all-lowercase
spellings, but not Yes/No. KVN SOLAR_RAD_PRESSURE, EARTH_TIDES, and INTRACK_THRUST use the
shared case-insensitive YES/NO lookup. KVN OBJECT_TYPE accepts its all-uppercase and all-lowercase
spellings, not mixed case; XML uses its schema-listed spellings. Writers emit each field's
canonical token.

XML whitespace around ORIGINATOR, MESSAGE_ID, OBJECT_DESIGNATOR, CATALOG_NAME, and
INTERNATIONAL_DESIGNATOR is trimmed on input. Embedded blanks such as `ESA SST` remain part
of ORIGINATOR; the other four identifiers still reject embedded blanks. Edited models with
padded identifiers are refused with that field named.
CDM 1.4.3.1's N/A convention applies only to CDM XML; other message families retain literal
XML text. The CDM reader checks fixed unit values before omitting N/A leaves. Wrong units report
`parse.xml.syntax` with a source location, including when XML comments or N/A normalization
are present. The XML writer
refuses standard scalar N/A values that would be lost on read-back, preserving comments and
MANEUVERABLE=N/A. NumPy symmetry is exact, without implicit averaging or a tolerance.

## KVN units and producer compatibility

CDM §6.2.4.1 requires applicable units to be displayed. KVN generation always writes each
unit-bearing field's fixed unit, including MISS_DISTANCE, relative geometry, screening sizes,
OD spans and percentages, areas, mass, state, and covariance fields. Omitted unit annotations
on KVN input are accepted as an unambiguous producer extension; the unit is defined by the
keyword and no conversion occurs. The same applies to XSD-optional XML annotations, but
XML fields whose schema requires units still require them. Rewriting such a model to KVN
therefore populates fixed-unit annotations. Dimensionless fields remain unitless.
`generation::kvn_generation_supplies_fixed_units_missing_from_the_model` covers both output
paths, reparsing, and generated XML schema validity. `optional_values` also checks empty/nil
unit validation and decoded empty scalar normalization.

## XML output and schema baseline

CDM 6.2.2 (lines of at most 254 ASCII characters, printable ASCII only) applies to both
notations. XML generation writes one indented element per line and refuses non-ASCII text and
lines longer than 254 characters before any caller-visible output; a long scalar is refused
rather than split or truncated. This also applies to CDM constituents of combined NDM output,
which is buffered for the check. XML input stays lenient: overlong lines and non-ASCII text in
otherwise valid XML are accepted, because XML producers commonly emit a single line and the
document is still unambiguous; regeneration then enforces the limit.
`generation::cdm_xml_output_is_printable_ascii_with_bounded_lines` covers both output paths.

Present empty or whitespace-only optional strings (for example `MESSAGE_FOR`, `OPERATOR_PHONE`,
`COLLISION_PROBABILITY_METHOD`) are refused on XML output, like `N/A`, because the XML reader's
missing-value compatibility would read them back as absent; use None instead.
`generation::xml_generation_refuses_present_empty_optional_strings` covers this.

Exactly empty XML unit attributes (`units=""`) on unit-bearing fields are treated as an absent
unit annotation. This is a producer-compatibility exception to the XSD unit enumerations for the
18 SDS file `CDMExample_issue1458.xml`, which writes `<RESIDUALS_ACCEPTED units=""/>`. Only the
empty string is accepted; nonempty incompatible units still fail, including on unavailable leaves.
Output always writes the fixed unit. `parsing::xml_empty_units_attribute_is_an_absent_annotation`
covers empty, valued, and incompatible cases.

The XML baseline is the bundled NDM/XML 4.0.0 CDM 1.0 schema. 508.0-B-1 Cor.2 cites NDM/XML
2.0.0; the library uses 4.0.0 because it is the current published schema set, shared with the
other message families. The 2.0.0 CDM schema is not bundled, so whether the CDM content differs
between the two sets has not been determined; validation against 4.0.0 is not a claim of 2.0.0
conformance.
