# TDM 2.0 conformance

This inventory records maintainer evidence for standalone TDM 2.0. The
[support matrix](../support-matrix.md) is the user-facing statement of current support.

## Scope and authorities

- Message edition: TDM 2.0; semantic and KVN authority: CCSDS 503.0-B-2 with
  corrigendum 1 (`data/tdm_503x0b2c1.pdf`).
- XML authority: NDM/XML schema set 4.0.0 and TDM schema 2.0.
- Profile: strict standalone parsing, validation, deterministic KVN/XML generation, and notation
  conversion through the shared Rust generation gate.

## Executable evidence

The TDM suites establish:

| Concern | Evidence |
| --- | --- |
| KVN structure | A local state scanner enforces printable ASCII outside comment payloads, header and `META`/`DATA` section sequencing, metadata uniqueness and the `PATH` choice, known observation keywords, and section-scoped comments. Metadata assignments are accepted out of order as a producer-compatibility decision; TDM §3.3.1.8 prescribes table order, which the writer emits. The four header keywords, by contrast, must keep table 3-2 order: the header is short, fixed, and no known producer reorders it, so no tolerance was added. Duplicate assignments and misplaced comments remain errors. |
| XML structure | The shared XML sequence engine covers the root, repeated segments, metadata, data, and the complete observation choice; it rejects unknown, duplicate, reordered, and non-schema nested content while accepting the schema's optional angle/percentage units. |
| Preservation and generation | All 21 shipped KVN fixtures and both shipped XML fixtures preserve their typed source-notation model. Generated XML validates against the official 4.0.0 master schema. Numeric domains have direct validation tests in addition to the [shared XSD oracle](../design/validation-contract.md#xsd-oracle-policy). Fixed optional XML units are semantically normalized when crossing unitless KVN. Bare `COMMENT` records are preserved by the shared KVN writer. |
| History allocation | `tdm_kvn_allocations` compares 10 and 1,000 observations. Parsing allocations grow with vector capacity rather than per record; validated streaming generation has fixed temporary-allocation overhead; materialized storage remains output-proportional. |
| Shared surfaces | `library::parsing` and `library::output`, `conformance::generation`, Python API tests, and the family benchmark matrices exercise shared parsing, generation, diagnostics, and dispatch. |

The existing exhaustive TDM unit tests exercise every observation variant and the indexed metadata
families. `tdm_kvn_history_scaling` provides reproducible Criterion workloads at 100, 10,000 and
50,000 observations and `tdm_xml_history_scaling` at the first two, the sizes shared by every
history-carrying family.

## Complete ICS reconciliation

All 53 numbered features in CCSDS 503.0-B-2 corrigendum 1 annex A2.1.5 were reconciled:

| ICS items | Logical requirement set | Reconciled implementation/evidence |
| --- | --- | --- |
| 1-6 | Header/version fields | `Tdm`, `TdmHeader`, strict root/version/epoch tests |
| 7-16 | Metadata preamble, participants, mode and path choice | `TdmMetadata`, indexed-field and conditional path/mode tests |
| 17-38 | Ephemerides, bands, turnaround, integration/range/angle/interpolation, delays, quality and corrections | complete optional metadata fixture/exhaustive tests, strict known-key scanner, unit normalization, and numeric domains enforced per segment: positive `INTEGRATION_INTERVAL` and `DOPPLER_COUNT_BIAS`, non-negative `RANGE_MODULUS` and all ten indexed transmit/receive delays, and finite `FREQ_OFFSET` and `CORRECTION_*` values |
| 39 | Metadata delimiter | local KVN state scanner and XML segment sequence |
| 40-41 | Data section and comments | strict section scanner and comment preservation tests |
| 42-52 | Every angle, carrier, clock, Doppler, media, meteorological, optical/radar, range, receive, transmit, and VLBI observation family | exhaustive typed observation-choice tests plus all 23 shipped fixtures, and observation value domains enforced through every segment: universal finiteness, `RHUMIDITY` in `[0, 100]`, positive transmit frequencies, non-negative tropospheric delays, `ANGLE_1`/`ANGLE_2` in `[-180, 360)`, and positive `RCS`, `STEC`, and `TEMPERATURE` |
| 53 | Data delimiter | local KVN state scanner and repeated XML segment sequence |

No annex-A TDM feature is absent. TDM's exception to the shared policy on [externally governed values](../design/validation-contract.md#registry-governed-values) is that normative
annex B places time-system and reference-frame registry membership outside the message, so those
values remain caller supplied. The implemented local syntax and conditional checks, and
the deliberate signal-context exceptions below, define the acceptance boundary.
Concretely, every populated `PATH`, `PATH_1`, and `PATH_2` is re-parsed at
validation and each index is checked against the `PARTICIPANT_n` fields actually supplied, so the
XML and tuple-field routes no longer bypass the path parser; and the `SINGLE_DIFF` requirement for
`RECEIVE_BAND` depends on whether the segment actually carries the non-indexed `RECEIVE_FREQ` or
`RANGE` observations that §§3.3.2.5.5 and 3.3.2.5.7 use for differenced data, rather than on
metadata alone. All six schema spellings of `RANGE_UNITS` (`km`, `KM`, `s`, `S`, `RU`, `ru`) are accepted
in XML. KVN reads `RANGE_UNITS` in any letter case (`Km`, `rU`): table 3-3 lists the three values
without a case rule, and each case variant names exactly one unit. Every writer emits the book
spellings `km`, `s`, and `RU`. This is a TDM alias set in the sense of the shared
[case rule](../design/validation-contract.md#shared-lexical-decisions).

## Packaged-surface evidence

`test_tdm.py` and the shared Python workflow matrix cover both
notations, file IO, epoch rules, generic identity, and diagnostics. Strict binding
audit, generated stubs/doc checks, wheel verification, and Rust artifact verification are the
packaged gates.

## Metadata vocabularies and exact phase counts

Table 3-3 permits ICD-defined `ANGLE_TYPE` values and delegates `REFERENCE_FRAME` to a living
SANA registry. KVN preserves nonempty caller-supplied values in those domains, including frames
outside the older XML enumeration. XML input/output uses the bundled schema's value domains;
conversion refuses an unrepresentable value before output instead of substituting a frame or
angle convention. Known XML enumeration values accept the uppercase/lowercase forms listed by
the XSD. Compatibility frame aliases `ITRF-2000`, `ITRF1993`, `ITRF93`, `ITRF1997`, `ITRF97`, and `TOD_EARTH` are also
accepted in XML and generate as `ITRF2000`, `ITRF-93`, `ITRF-93`, `ITRF-97`, `ITRF-97`, and `TOD`, respectively.
Arbitrary mixed case is rejected in XML. KVN reads the same aliases and the book spellings
`ITRF1993` and `TOD_EARTH` as the known frames, so KVN output is canonical too: `ITRF1993`
and `ITRF93` are written `ITRF-93`, `ITRF-2000` is written `ITRF2000`, and `TOD_EARTH` is
written `TOD`. Known mixed-case aliases canonicalize too; only open values outside the known
frames, such as `GCRF`, retain their spelling in KVN.

`REFERENCE_FRAME` is required whenever `ANGLE_TYPE` is `RADEC`, in any letter case (a mixed-case
`Radec` canonicalizes to `RADEC` and triggers the rule). Table 3-3 says the frame "shall
be used in conjunction with" RADEC, and right ascension/declination angles cannot be interpreted
without their inertial frame; the XSD does not express the dependency. `DATA_TYPES`, when supplied, must contain observation keywords from table 3-5;
indexed families may use the table's `_n` notation. The library does not invent participant
identities, calibration truth, or an ICD for a custom angle convention.

Sections 3.5.2.11–3.5.2.12 define phase counts as real-number strings with any required number of
digits, and 4.3.11 adds that the KVN string "must not contain any alphabetic or special
characters". The XSD types the same elements as `xsd:double`, whose lexical space includes
exponents. Both authorities are kept per notation: KVN input rejects an exponent (`1.5E3`) in a
phase count; XML input accepts and retains it, trimmed of surrounding blanks like every XML
number. Such a count cannot be written to KVN without rewriting its digits, so KVN generation and
XML-to-KVN conversion refuse it, naming the keyword, rather than expanding the exponent.
`parsing::phase_count_exponents_are_xml_double_spellings_but_not_kvn_phase_counts` covers all
three paths. All ten receive/transmit phase-count variants retain their source text in the model,
XML and Python. Decimal counts also preserve it through KVN and conversion, except that negative lexical
zero is written as `0` in KVN. They are not parsed
through `f64`. Lexical validation rejects
malformed numbers and non-finite spellings without limiting the number of digits. Other
observations use doubles and the ordinary-number generation rules below. The allocation-growth
budget uses ordinary numeric observations; retaining each exact phase count requires storage
proportional to its text.
`tdm::model` tests exact counts above `2**53`, long fractional parts, very large exponents, invalid
lexemes, metadata vocabularies, and zero-byte XML refusals; Python tests exercise all ten variants.
The exact decimal model and XML do not impose a precision limit. KVN output still applies its separate
254-character line limit: a phase-count record that exceeds it is refused, never truncated.

For ordinary numeric observations and metadata, TDM sections 4.3.4–4.3.5 limit fixed-point
digits and floating-point mantissas to 16 digits. KVN generation uses the shared numeric
formatter to round when necessary; values whose rounded spelling would overflow a double
are refused before output. XML retains the original double value. The bundled TDM XSD
types ordinary observations and phase counts as doubles, but the more specific book
sections 3.5.2.11–3.5.2.12 require arbitrary-precision phase-count strings. Those counts keep
their validated decimal text rather than passing through the ordinary-number formatter.
`tdm::generation::kvn_numbers_follow_the_digit_limit_without_rounding_phase_counts` and
`kvn_refuses_numbers_whose_rounded_spelling_overflows_before_output` cover both boundaries.

## Verification outcome

TDM 2.0 meets the common [verification criteria](README.md#verification-criteria) for
self-contained format and model validation on Rust and Python. The requirement inventory,
family tests, shared public-route tests, and packaged tests are complementary evidence;
none alone establishes conformance or exhaustive mutation coverage.

## Decisions and limits

**Observation order is per keyword.** Sections 3.4.10–3.4.11 require strictly increasing,
non-repeated epochs for each observation keyword within a segment. Different keywords may
interleave in any order, including a decrease in the overall row sequence. Validation checks
each keyword separately and generation preserves producer row order; it never sorts records.
Indexed phase counts (`RECEIVE_PHASE_CT_n`, `TRANSMIT_PHASE_CT_n`) require the corresponding
`PARTICIPANT_n`, because §§3.5.2.11–3.5.2.12 index them "to enable association with the
PARTICIPANT_n", just as PATH references require a populated participant. The indexed
frequencies are not tied to a participant: §3.5.2.8 indexes `RECEIVE_FREQ_n` by downlink and
§3.5.2.9 indexes `TRANSMIT_FREQ_n` (and so `TRANSMIT_FREQ_RATE_n`) by transmitter, so they need no
`PARTICIPANT_n`. A participant or `RECEIVE_BAND` whose value is blank (an empty XML element) does
not count as populated for the PATH, phase-count, or `SINGLE_DIFF` rules.
`epochs::observations_are_ordered_per_keyword_without_reordering_other_keywords` and
`model::doppler_counts_are_integral_and_only_phase_counts_require_their_participant` cover
these rules through parsing and generation.

`PATH`, `PATH_1`, and `PATH_2` are comma lists with "no inserted white space" (table 3-3). KVN
input with blanks around the commas (`PATH = 2, 1`, as Orekit writes it) is read as the canonical
`2,1`; the blanks carry no information, so this is a lossless producer tolerance under the
[shared policy](../design/validation-contract.md#authority-and-representability). Writers emit
`2,1`. XML follows the schema pattern `\d,\d(,\d)*` and model values must already be canonical.
`parsing::kvn_paths_with_blanks_after_commas_read_as_canonical_paths` covers it.

A lone `PATH_1` or `PATH_2` is rejected in every mode, although the XSD makes each element
optional on its own. Table 3-3 defines the pair as the two legs of differenced (`SINGLE_DIFF`)
data, and a single signal path is exactly what `PATH` expresses, so a lone leg describes no
signal path the book defines.

The published Annex E17 example repeats the RCS epoch `2011-05-11T10:26:33.7008` for two
different measurements, contrary to section 3.4.11. Its original fixture is retained unchanged
as `data/invalid/tdm_e17_original.kvn` and rejected. The annotated positive fixture adapts the
last RCS epoch to `2011-05-11T10:26:33.9686`, matching the other measurements in that group.
That repair is a test-fixture choice, not a correction published by CCSDS or a parser rewrite.
`parsing::original_e17_repeated_rcs_epoch_is_rejected_and_adaptation_remains_valid` covers both.

**Doppler counts are integral doubles.** Section 3.5.2.4 requires `DOPPLER_COUNT` to be an
integer, although the XML schema declares an unrestricted double. The public numeric type
remains `f64`; fractional, non-finite counts are rejected on both notations and generation.
Positive counts are only recommended, so signed integer counts and zero remain accepted.
This count is distinct from the arbitrary-precision decimal phase-count strings described
above. Values already rounded by a caller into a double cannot recover lost integer digits.

**Epoch spellings follow the book.** XML epochs with a nonzero timezone offset, bare numeric epochs, and
empty epochs are rejected although the XSD `epochType` admits them; this is the shared
book-over-XSD decision stated once in the
[validation contract](../design/validation-contract.md#authority-and-representability).

**Epoch storage has a library limit.** All epoch tokens, including observation epochs, are
limited to 64 bytes before parsing. This is not a CCSDS or XSD length restriction. Longer
tokens are rejected, never truncated, on both notations and in direct epoch construction.
Other field-specific precision rules still apply where documented. See the shared
[epoch storage policy](../design/validation-contract.md#epoch-storage-limit).

The primary implementation and full output evidence cover edition 2.0. A restricted
input-only 1.0 reader is also supported in KVN and XML; see [TDM 1.0 input](#tdm-10-input).

Time-system and frame registry membership, calibration truth, and the correctness of an
exchange-partner path are outside the message. Local participant-index existence, path/mode
choices, observation domains, and conditional metadata remain library checks.

Metadata `START_TIME`/`STOP_TIME` are checked for calendar syntax and the shared 64-byte storage
limit; no fractional-digit cap applies. The library does not compare their ordering or check
that they enclose every observation. Table 3-3
describes these bounds in UTC while observations use `TIME_SYSTEM`; observation enclosure
can require time-scale conversion, which the library does not perform. The XSD permits
independently optional bounds. Callers must establish coverage and window consistency;
the per-keyword observation chronology checks do not establish those properties.

Shared Rust/Python route coverage, including `test_conformance_matrix.py`, and installed
artifact checks are indexed once in the [common evidence table](README.md#shared-evidence).

All four book line endings (LF, CR, CRLF and LFCR) are accepted with identical models and
byte-position-preserving normalization.

## Stricter than XSD

These TDM 2.0 book domains are narrower than the bundled TDM 2.0 XSD. The book rule applies in
KVN, XML, and model validation alike, to keep that value domain compatible across notations:

| Field | Book (503.0-B-2) | XSD (`ndmxml-4.0.0-tdm-2.0.xsd`) |
| --- | --- | --- |
| `ANGLE_1`, `ANGLE_2` | §§3.5.4.2–3.5.4.3: `-180 <= angle < 360` | `ndm:angleType`: `[-360, 360)` |
| `RCS` | §3.5.5.2: positive | `xsd:double` |
| `STEC` | §3.5.7.1: positive | `xsd:double` |
| `TEMPERATURE` | §3.5.8.3: positive | `ndm:nonNegativeDouble` (admits 0) |
| `DOPPLER_COUNT` | §3.5.2.4: integer | `xsd:double` |
| KVN phase counts | §4.3.11: no alphabetic characters | `xsd:double` (exponents allowed; kept in XML, see above) |

Where book and XSD agree the bound also applies in every notation, under the shared
[facet rule](../design/validation-contract.md#authority-and-representability): positive
`TRANSMIT_FREQ_n` (§3.5.2.9 and `ndm:positiveDouble`),
`RHUMIDITY` in `[0, 100]` (`ndm:percentageTypeUO`; the range error names `RHUMIDITY`), non-negative `TROPO_DRY`/`TROPO_WET` (`ndm:nonNegativeDouble`), and positive
`INTERPOLATION_DEGREE` and `DOPPLER_COUNT_SCALE` (`xsd:positiveInteger`).
`generation::tdm_observation_domains_follow_the_narrower_book_range` and
`generation::unambiguous_tdm_observation_domains_are_revalidated` cover them.

## KVN line length

TDM 4.2.1 limits a KVN line to 254 characters and defines no continuation lines. Longer lines are
read (Orekit writes a 708-character `DATA_TYPES` line), as the
[shared lexical rules](../design/validation-contract.md#shared-lexical-decisions) allow. KVN
generation refuses any `KEY = value` record or phase-count observation that would exceed the
limit, naming the keyword (for example `DATA_TYPES`); it never wraps or truncates. XML has no line
limit, so such a message still converts to XML.
`generation::kvn_output_refuses_integers_and_lines_beyond_the_kvn_limits` covers it.

## Observation units

Optional XML `ANGLE_1`/`ANGLE_2` unit annotations are checked against the schema's `deg`
spelling; `rad` is rejected rather than ignored or converted. Their fixed degree annotation
may be omitted by the writer. KVN observation numbers and ordinary XML observations such as
`RANGE` and `DOPPLER_INSTANTANEOUS` use the shared finite-number reader, including
nonzero-underflow rejection. XML `ANGLE_1`, `ANGLE_2`, and `RHUMIDITY` use the unit-bearing
double reader: `1e-999` and `-1e-999` round to positive and negative zero respectively,
following XML double value mapping. Both paths retain representable subnormals such as `5e-324`.
Exact phase counts retain their separate decimal model.
`tdm_xml_accepts_the_schema_defined_optional_observation_units` covers incompatible annotations,
and `observation_numbers_use_shared_reading_rules_without_underflow` covers observation spelling
and range boundaries.
`xml_unit_bearing_observations_follow_double_underflow_mapping` records the XML distinction.

## XML default elements and numeric storage

The bundled TDM schema assigns zero defaults to present, empty `FREQ_OFFSET`, `RANGE_MODULUS`,
all five `TRANSMIT_DELAY_*` and all five `RECEIVE_DELAY_*` elements, and observation
`CLOCK_BIAS`/`CLOCK_DRIFT`. Empty `DATA_QUALITY` means `RAW`. Readers materialize those
values; writers emit them explicitly. Omitted optional metadata remains absent rather
than being populated. Nonempty XML `DATA_QUALITY` values use only the exact schema
spellings (`RAW`/`raw`, `VALIDATED`/`validated`, `DEGRADED`/`degraded`); mixed case and
padding are rejected like other schema string enumerations. KVN remains case-insensitive.
`xml_data_quality_uses_schema_spellings_while_kvn_remains_case_insensitive` covers this boundary.
Empty elements without a schema default remain invalid.
`tdm::parsing::present_empty_xml_elements_use_schema_defaults_and_omissions_stay_absent`
checks every default-bearing field, both empty element spellings, and nearby invalid cases.

**Metadata whitespace exception:** `FREQ_OFFSET`, `RANGE_MODULUS`, `TRANSMIT_DELAY_1` through
`TRANSMIT_DELAY_5`, and `RECEIVE_DELAY_1` through `RECEIVE_DELAY_5` also read whitespace-only
XML content as zero, including character references and CDATA. This is a library limitation,
not an XSD default rule or a known-producer requirement: quick-xml drops whitespace-only text
while buffering these metadata elements, before their default handler sees it. Preserving the
distinction would require additional XML-specific parsing while retaining numeric model
deserialization. Writers emit explicit zeroes. `CLOCK_BIAS` and `CLOCK_DRIFT` use a direct
text reader and reject whitespace-only content; only truly empty elements default to zero.
`DATA_QUALITY` retains its exact string-domain rules above. The same regression checks all
twelve metadata fields and the stricter clock observations.

The [integer storage limits](../design/validation-contract.md#integer-storage-limits) apply:
`INTERPOLATION_DEGREE` is positive `u32` and `DOPPLER_COUNT_SCALE` is positive `u64`,
although their XSD positive-integer types have no upper bound. KVN integers are limited to
2³¹−1 by TDM 4.3.2, so KVN generation refuses either value above
`i32::MAX`, naming the keyword; XML keeps the full storage range. Exact phase counts remain
strings. Schema-listed lowercase metadata forms are covered by
`tdm::parsing::xml_accepts_schema_listed_lowercase_categories`.

KVN reading tolerates lines longer than the book's 254-character limit where applicable;
writing enforces that limit, so an accepted overlong record can require shortening before
regeneration. Shared encoding, namespace, epoch, `N/A`, whitespace and categorical rules
are specified in the [validation contract](../design/validation-contract.md).


KVN closed categories ignore case (4.3.7), and blank/underscore aliases are equivalent (4.3.8).
DOPPLER_COUNT is emitted as plain integer digits (3.5.2.4), within the signed 32-bit
KVN integer range (4.3.2); out-of-range model values remain available in XML and are refused
before any KVN output. Exact phase counts retain arbitrary precision, with `.5` and `5.`
written as `0.5` and `5.0` (4.3.4). Negative zero writes as `0` for Doppler/phase counts;
ordinary floating observations write it as `0.0`. XML and the model retain their source sign.
PARTICIPANT_1 is mandatory because the XSD requires it, despite the book's broader
'at least one participant' wording. Indexed delay and ephemeris-name fields are preserved
without requiring that participant; phase-count observations do require their participant.
INTEGRATION_REF companion fields, REFERENCE_FRAME with AZEL, a zero turnaround denominator,
and whether interpolation is necessary for a changing uplink are not inferred/enforced:
inapplicable descriptors are retained rather than discarded. Whether interpolation is needed
for a changing uplink additionally depends on the signal path and ICD. The library performs
no turnaround-ratio calculation, so the schema's zero integer value is preserved. Negative
zero denotes the same measurement as zero, but its sign bit remains in model/XML values;
only KVN generation canonicalizes its spelling (4.3.5).
Python nested TDM records are validated through Tdm.validate(); only the Rust API currently
exposes each nested validate method separately.

Recognized categorical frames and angle types canonicalize on KVN input, including
`Eme2000` to `EME2000` and `Radec` to `RADEC`; arbitrary open registry text is retained only
where the model field is open. `legacy_tables_exclude_every_later_edition_metadata_keyword`
covers the 1.0 table boundary.

## Signal context and construction boundaries

DOR and VLBI_DELAY are accepted without an in-message `TRANSMIT_FREQ_n` history or a complete
signal/reference path. This deliberately follows the schema's permissive structure rather than
book §§3.5.3.2–3.5.3.3's requirement to supply the beacon/uplink or interferometer reference
frequency. Callers must establish that frequency and signal context; validation does not certify
that the measurements can be reconstructed from this message alone.
Two-way and three-way differenced Doppler is different: §3.3.2.5.6's trigger is decidable from
the message, so it is enforced (see [below](#additional-input-and-signal-context-boundaries)).
`DATA_TYPES` checks keyword vocabulary, not correspondence with actual observations. Angle
observations may omit `ANGLE_TYPE`; custom conventions remain the exchange partners' context.
When MODE is absent, either PATH form is retained and no mode-specific companion rule is inferred.

Present but empty XML/model `DATA_TYPES` is invalid: the declaration must name one or more
observation keywords. A blank optional KVN DATA_TYPES record reads as absent under the shared
missing-value rule. Empty list members and trailing commas are rejected. This is field-specific validation,
not a rule that all optional strings are interchangeable. Implemented shared numeric/conditional
checks also apply to restricted 1.0 input. These checks have not been independently reconciled
against the unavailable TDM 1.0 book; the bundled 1.0 XSD is the reproducible XML authority.

Phase-count text accepts a sign and preserves leading zeros and decimal digits. KVN output
fills a missing digit beside a decimal point and canonicalizes negative zero; exponent text
retained from XML is refused by KVN output rather than expanded or rounded. No 32-bit or double
precision bound is imposed on these strings.

In Python, omitted optional metadata getters return `None`, including fields with book defaults;
those defaults describe interpretation, not automatic constructor population. Categorical metadata
setters (`mode`, path fields, timetag/integration/range references, units, angle/frame values,
yes/no flags and quality) can raise `ValueError` immediately. Invalid epoch tokens raise
`NdmEpochError`. `TdmObservation` construction resolves the keyword, epoch and lexical number;
RHUMIDITY additionally checks its percentage domain immediately. Complete observation and
cross-field domains are checked by root `validate()` and every generation route. Rust also
exposes nested validators; Python currently exposes root validation.
Observation values use the fixed per-keyword units in table 3-5 and §§3.5.1–3.5.8; RANGE uses
`RANGE_UNITS`. There is no unit conversion. Phase counts should be supplied as decimal strings
when every digit matters.

## TDM 1.0 input

TDM 1.0 (CCSDS 503.0-B-1, November 2007 with Cor. 1 of September 2010,
https://public.ccsds.org/Pubs/503x0b1s.pdf) is read in KVN and XML through a restricted
input-only reader. The message keeps `version = "1.0"`; KVN and XML generation and conversion
refuse it with `generation.unsupported_output_version`. The reader never relabels the edition;
an explicit caller relabelling to 2.0 is possible after validation.

**What changed between 1.0 and 2.0.** Edition 2.0 only adds keywords; nothing was removed or
renamed, and the shared keywords keep their order, units, defaults, value domains, conditional
rules, chronology and uniqueness rules (§§3.4.10–3.4.11) and chapter 4 syntax. The 1.0 reader
therefore reuses the 2.0 structure and rejects every 2.0 addition: header `MESSAGE_ID`;
metadata `TRACK_ID`, `DATA_TYPES`, `EPHEMERIS_NAME_n`, `INTERPOLATION`,
`INTERPOLATION_DEGREE`, `DOPPLER_COUNT_BIAS`/`SCALE`/`ROLLOVER`, `CORRECTION_MAG`,
`CORRECTION_RCS`, `CORRECTION_ABERRATION_YEARLY`/`DIURNAL`; observations `DOPPLER_COUNT`,
`MAG`, `RCS` and the phase counts. Every 2.0 value domain and conditional rule that runs on 1.0
input is reused as an explicit compatibility profile. Independent confirmation against the
1.0 book remains unavailable; no equivalence with that book is claimed.

**XML follows the NDM/XML 1.0 schema.** NDM/XML 505.0-B-1 (December 2010,
https://public.ccsds.org/Pubs/505x0b1s.pdf) defined TDM 1.0 XML in
`ndmxml-1.0-tdm-1.0.xsd`. That schema is no longer hosted by SANA; the bundled copies
(`ndmxml-1.0-tdm-1.0.xsd`, `ndmxml-1.0-navwg-common.xsd`, `ndmxml-1.0-ccsds-common.xsd`, labelled
"version 1.0 … 03/14/2010") come from the only Internet Archive capture of
sanaregistry.org/r/ndmxml/ (2011-12-06). Its element names, nesting, order and enumerations
match the 2.0 schema apart from the additions above, with three narrower rules the reader
enforces for 1.0 XML:

| 1.0 schema rule | Library behaviour |
| --- | --- |
| `TIME_SYSTEM` is the closed `timeSystemType` list: GMST, GPS, SCLK, TAI, TCB, TDB, TOD, TT, UT1, UTC, each all-uppercase or all-lowercase | Other values (including `TCG` and mixed case such as `Utc`) are rejected in 1.0 XML |
| `RHUMIDITY` is `percentType` with no `units` attribute | `units` on `RHUMIDITY` is rejected in 1.0 XML |
| Only the unqualified schema set exists | A namespaced (qualified) root or descendant element is rejected in 1.0 XML |

**KVN and XML differ on TIME_SYSTEM by design.** The restricted KVN compatibility profile keeps `TIME_SYSTEM` open, as 2.0 does;
this has not been independently checked against the unavailable 1.0 book. The 1.0 XML schema closes the list, and XML follows its schema. A 1.0 KVN file using
an ICD-defined time system therefore reads, while the same content in 1.0 XML is rejected.

**Evidence.** The repository's legacy annex-D example corpus was checked: 11 of 15 read; the other four contain
invalid records in the retained samples (duplicate keyword/epoch pairs in D-4 and D-5, a `CREATION_DATE`
without seconds in D-7, and `PARTICIPANT 3` without its underscore in D-10), three of which
2.0 corrected. Every schema-valid 1.0 XML sample in the repository reads; the two that fail are
also invalid under the 1.0 schema (`TIME_SYSTEM` `TCG` and `WRONG-TIME-SYSTEM`). The 1.0
combined `<ndm>` envelope accepts supported 1.0 constituents as input only.

Tests: `conformance::tdm1::tdm_1_0_xml_input_follows_the_ndm_xml_1_0_schema` validates the
505.0-B-1 figure B-9 fixture (`data/legacy/tdm_1_0_b9.xml`) against the bundled 1.0 schema and
checks that each rejection above is also a schema rejection;
`tdm::parsing::legacy_kvn_is_input_only_and_rejects_version_two_content` covers typed/generic
input, refused output and later-edition content; and
`legacy_tables_exclude_every_later_edition_metadata_keyword` covers the 1.0 table boundary.

## Additional input and signal-context boundaries

§3.5.2.4 requires an integer DOPPLER_COUNT and §4.3.2 gives KVN integers a signed 32-bit
range. KVN input nevertheless accepts integral finite double spellings such as `1000.0` and
`1.0E3`, and integral values beyond 2^31: cycle-count accumulators routinely exceed 32 bits and
these spellings name the same integer, so reading them loses nothing. A non-integral count such
as `1000.5` is rejected on every route, never truncated. XML uses the XSD's double domain with the
same integral rule. KVN output is strict: it writes integer digits and refuses values outside the
signed 32-bit range (use XML for those), so not every accepted count can be regenerated as KVN.

XML ANGLE_TYPE and REFERENCE_FRAME use exact schema spellings and the explicitly recorded
frame aliases. KVN padding and blank/underscore aliases are not XML enum spellings.
`pre_release::tdm_xml_does_not_accept_kvn_category_padding_or_blank_aliases` checks that boundary.

TDM §3.3.2.5.6 requires a `TRANSMIT_FREQ_n` uplink-frequency history when the TDM contains
two-way or three-way differenced Doppler. Validation enforces it at message level: when any
segment has `MODE = SINGLE_DIFF`, `RECEIVE_FREQ` observations (the differenced-Doppler observable,
§3.3.2.5.5), and a `PATH_1` or `PATH_2` of three participants (one uplink and one downlink leg,
§3.3.2.4.3 notation, e.g. `2,1,2` or `2,1,3`), some segment of the message must carry a
`TRANSMIT_FREQ_n` observation. One-way paths such as Figure E-10's `1,2`/`1,3` are not affected.
The library does not establish that the supplied series covers the observation interval or is
sufficient for processing; that remains signal-processing context, and missing frequencies are
never inferred.

The implemented RECEIVE_BAND condition is SINGLE_DIFF with RECEIVE_FREQ or RANGE observations;
metadata alone does not trigger it. Other frequency-family observables and whether the supplied
band is physically appropriate remain exchange-partner context; validation does not certify
frequency-dependent corrections.
