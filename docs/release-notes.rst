Release notes
=============

Unreleased
----------

Pre-release conformance fixes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Data loss and output correctness:

- OCM KVN covariance rows that fail to parse (``abc``, underflow such as ``1e-400``) now fail
  the parse instead of being silently dropped.
- KVN unit annotations without a value (``MASS = [kg]``, ``MASS = N/A [kg]``) are errors in every
  family instead of being read as absent with the unit discarded (ODM 7.7.1.1, CDM 6.3.3).
- CDM XML output is written one element per line and satisfies CDM 6.2.2: non-ASCII text and
  records over 254 characters are refused before any bytes are written, also inside combined NDM
  output. CDM XML generation refuses present empty or whitespace-only optional strings instead of
  losing them. XML input stays lenient.
- ACM KVN output writes ATT/COV time tags with a digit on both sides of the decimal point
  (``.5`` → ``0.5``) and refuses tags over 16 digits (ADM 6.8.4.1) instead of copying them. XML
  keeps the stored spelling.
- OCM output writes ``MAN_COMPOSITION`` as uppercase comma-separated names and ``TRAJ_UNITS``,
  ``COV_UNITS`` and ``MAN_UNITS`` as bracketed comma-separated lists in KVN and XML (ODM 6.2.8.15,
  tables 6-4/6-6/6-7). Reading still accepts blank-separated and unbracketed forms.
- The TLE helper (``Omm.from_tle_lines``) emits OMM 3.0 (was 2.0, which became invalid when
  ``message_id`` was given). Its CREATION_DATE (TLE epoch) and ORIGINATOR/OBJECT_NAME (``UNKNOWN``)
  defaults are documented placeholders. ``to_tle_lines`` writes the table 4-3 defaults U/0 when
  CLASSIFICATION_TYPE/EPHEMERIS_TYPE are omitted.

Book requirements now enforced:

- OMM with ``MEAN_ELEMENT_THEORY`` SGP, SGP4 or SGP/SGP4 requires CENTER_NAME EARTH, REF_FRAME
  TEME and TIME_SYSTEM UTC (ODM 4.2.4.6) in parsing, validation and generation (breaking).
- TDM requires a ``TRANSMIT_FREQ_n`` history when a message contains two-way or three-way
  SINGLE_DIFF ``RECEIVE_FREQ`` data (TDM 3.3.2.5.6).
- CDM generation requires INTERNATIONAL_DESIGNATOR as ``YYYY-NNNP{PP}`` or ``UNKNOWN``
  (table 3-3, breaking for output); parsing still accepts producer values such as CARA's ``1001``.
- OCM rejects mixed-case ``MAN_COMPOSITION`` names (ODM 7.5.3); all-upper and all-lower remain.
- ACM/OCM USER blocks without a USER_DEFINED parameter are read (the XSD admits ``<user/>``) but
  refused by KVN and XML generation (ADM table 5-9, ODM table 6-12).
- RDM reads impact confidence regions without IMPACT_REF_FRAME (table 3-3 requires the frame only
  with a nominal location) but refuses to write their coordinates without a frame. RDM XML input
  rejects ``NOMINAL_IMPACT_ALT`` outside the schema range, as XML output already did.
- TDM XML ``ANGLE_TYPE`` and ``REFERENCE_FRAME`` accept only exact schema spellings; KVN keeps
  its blank/underscore and case lookup.

Real-world input:

- NDM/XML 1.0 ``<ndm>`` envelopes (e.g. Space-Track) are read as input only, checked against the
  1.0 envelope schema: unqualified elements, no MESSAGE_ID, ``COMMENT*`` then
  ``aem/apm/oem/omm/opm/tdm``, constituents ODM 2.0 or TDM 1.0 (APM/AEM 1.0 stay rejected).
  Generation writes only the NDM/XML 4.0.0 envelope and refuses 1.0-era constituents.
- CDM XML input accepts exactly ``units=""`` as an absent unit annotation (18 SDS producer case);
  nonempty incompatible units are still rejected.
- ACM metadata CLASSIFICATION (ADM table 5-3) is read, stored (``AcmMetadata.classification``,
  also in Python) and written in KVN; XML generation refuses it because the schema has no element.
- APM/AEM KVN ``ATTITUDE_TYPE`` applies blank/underscore equivalence (``EULER ANGLE``).
- OEM: an empty or whitespace-only XML INTERPOLATION no longer requires INTERPOLATION_DEGREE (KVN
  output refuses the empty value). XML→KVN trims padding around ORIGINATOR, CLASSIFICATION and
  MESSAGE_ID and writes TABs in comments as blanks instead of failing.

API:

- Typed Rust parsing now supplies the same structured diagnostic context as generic parsing.
  Callers matching errors directly must account for ``CcsdsNdmError::Parsing``; the error
  accessors still expose the underlying cause. Invalid optional CDM enums point to their own
  values, and XML diagnostic locations count CR, LF and CRLF line endings correctly.
- Angle, day-interval, GM, mass and area deserialization shares the quantity reader's XML
  numeric spelling rules and rejects duplicate value or unit fields. XML namespace declarations
  reject malformed prefixes while retaining valid Unicode prefixes.
- Rust: ``AltitudeRequired`` is renamed ``ImpactAltitude``; its units remain optional.
  ``EnumParseError.field`` is ``Cow<'static, str>`` so KVN errors name the actual keyword. The unused
  ADM 1.0 ``ApmRateFrame`` enum is removed. RDM ``CONTROLLED_REENTRY`` errors name that keyword;
  APM comment errors cite ADM 6.10.2.
- Python: ``CombinedNdm.id`` is settable; ``AcmMetadata.classification`` is new.

Documentation:

- The validation contract has a table of unambiguous lexical variants (TAB, long KVN lines,
  ``7000[km]``, ``.5``, UTF-8 comments, ``±00:00``, BOM, XML declarations) giving for each the
  clause relaxed, accepted shape, canonical output and test. It also records that epoch checks are
  lexical (``ss=60`` at any minute), the free-text case deviation, and per-family COMMENT placement.
- Family records list known producer files the library rejects and why (Orekit OEM/ACM samples),
  the producer evidence for structural relaxations, the TDM ``DOPPLER_COUNT`` read tolerance, the
  OMM ``MEAN_MOTION`` reading for non-SGP4 theories, and the SANA Delaunay element order. Broken
  anchors, stale statements and Python docstrings are reconciled.

Final conformance corrections
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

- RDM KVN generation refuses confidence levels whose 16-digit rounding would collapse
  their required strict ordering, before writing bytes. XML retains the original precision.
- TLE export refuses launch years and rounded epoch years outside 1957–2056 instead of
  silently changing centuries, and rejects non-ASCII launch identifiers without panicking.
  Ordinary OMM validation and XML retain their wider date and identifier domains.
- OCM XML now reads back empty user-defined values. The validation contract records the shared
  XML reader's whitespace-only-to-empty normalization; padding around nonblank values remains
  preserved. ACM rate-only ATT_TYPE errors cite ADM table 5-4.

- OCM rejects positive ``DEPLOY_MASS`` text even when its magnitude underflows to double
  zero. Negative underflow text remains supported in the model/XML.
- Documentation clarifies mandatory root identifiers, file-encoding exception categories,
  numeric rounding and NaN checks, and edition-specific OMM theory requirements.
- OCM validates ``OD_EPOCH`` and supplied ``SW_DATA_EPOCH`` as real calendar/ordinal dates
  or complete relative numbers in parsing, model validation, and generation. Empty supplied
  XML/model epochs, impossible dates, and incomplete numbers are rejected; zero and negative
  relative tags remain supported. Blank optional KVN values still mean absent.
- The shared contract documents multiline-comment splitting versus CDM/TDM KVN refusal;
  OMM comment preservation and TDM's uncapped fractional-second precision are clarified.
- ACM KVN generation refuses explicitly empty ``SENSOR_NOISE_STDDEV`` values rather than
  losing them; XML retains them. Multiline comments use the shared writer in every ACM block,
  splitting into separate KVN records while retaining ASCII checks and streaming preflight.
- OCM requires nonblank ``DC_BODY_FRAME`` for ``TIME_AND_ANGLE`` and rejects the non-schema
  ``units`` attribute on ``WEIGHTED_RMS``. Its documentation now distinguishes Unicode-comment
  input acceptance from KVN output refusal.
- Rust ``SigmaU``/``SigmaV`` documentation now identifies rate/angle random walk and cites
  ADM table 5-8. The sensor-noise citation is corrected to the same table.
- TDM documents and tests its reader limitation: whitespace-only content defaults to zero
  in twelve numeric metadata elements, while clock observations require truly empty elements.

- Combined XML generation rejects TDM 1.0 constituents under the current envelope's TDM 2.0 schema;
  standalone TDM 1.0 input remains supported.
- ACM requires nonblank ``CP_REF_FRAME`` and ``TARGET_MOM_FRAME`` when their vectors are supplied.
- OCM checks the required ``n/a`` maneuver-unit labels for switches, dimensionless and text columns.
- OMM ``ELEMENT_SET_NO`` accepts XML signed integer zero while retaining its 0–9999 range.
- Documentation records Unicode whitespace tolerance in XML numeric vectors, field-specific XML
  underflow behavior, and OCM clock defaults and descriptive data-element declarations.

- KVN rejects supplied units on OCM ``DC_REF_DIR``/``DC_BODY_TRIGGER`` and ACM
  ``TARGET_ATTITUDE`` instead of silently dropping them.
- Combined XML accepts whitespace character references and CDATA between envelope elements;
  these constructs remain invalid outside the document root.
- TDM XML ``DATA_QUALITY`` requires the schema's exact uppercase/lowercase spellings;
  mixed case and padded values are rejected. Empty elements still default to ``RAW``,
  and KVN remains case-insensitive.

- XML attribute values follow XML 1.0 whitespace normalization: literal TAB/LF/CR become
  spaces (CRLF counts once), while character references retain their characters. Output
  preserves TAB/LF/CR in attributes using references, including user-defined parameter names.
  Element-text whitespace is unchanged.
- Atomic file replacement applies existing destination permissions before writing contents,
  preventing private output from appearing in a more broadly readable temporary file.

- Python combined messages reject nested or self-referencing constituents before conversion,
  raising ``ValueError`` instead of recursing until the process crashes. XML constituent
  deserializer errors now report positions in the original combined document.
- ACM XML input/output checks exact schema spellings for ``ATT_BASIS``, ``COV_BASIS`` and
  ``AD_METHOD``. ACM metadata rejects impossible absolute dates and incomplete relative time
  tags while retaining zero and negative offsets. Assigning Python attitude rows derives their count and validates the replacement
  before changing the existing rows.
- CDM KVN always writes required fixed units. Empty/nil XML leaves validate supplied units
  before normalization, decoded empty CDATA follows the same rules as empty text, and restricted
  identifiers consistently ignore surrounding XML padding.
- RDM accepts omitted XML ``NOMINAL_IMPACT_ALT`` units. Rust ``ImpactAltitude.units`` is
  ``Option<LengthUnits>`` (breaking); Python numeric access is unchanged. KVN supplies ``[m]``.
- OPM preserves repeated user-defined names through KVN/XML. Empty parameter-name suffixes are
  rejected on input; empty and literal ``N/A`` user-defined values remain opaque payloads.
- OMM XML accepts signed integer zero according to XSD value equality. KVN ``DOPPLER_COUNT``
  output enforces the signed 32-bit integer limit and writes zero without a negative sign.
  Exact phase counts retain arbitrary precision; only signed-zero spelling is normalized in KVN.
- All KVN readers accept a missing final newline and printable Unicode in COMMENT payloads.
  XML preserves those comments; ASCII KVN output refuses them explicitly. Zero epoch offsets
  ``+00:00`` and ``-00:00`` normalize to ``Z`` without converting the declared time system.
  Nonzero offsets and the existing fractional-second precision limits remain unchanged.
- Python ``OpmCovarianceMatrix`` takes ``comment=`` rather than ``comments=`` (breaking),
  matching its property. AEM compares object names using blank/underscore equivalence and
  refuses obsolete symbolic ``RATE_FRAME`` placeholders instead of treating them as frame names.
- Reserved values apply blank/underscore equivalence (ODM 7.5.9, ADM 6.8.8) wherever they are
  compared: OCM ``MAN_COMPOSITION`` reads complete column aliases such as ``THR X``, a padded
  ``MAN_DEVICE_ID = DEPLOY`` is the reserved deployment device, and known trajectory/covariance
  types such as ``TEQUINOCTIAL P`` keep their matrix checks. ACM ``MOM DESAT``-style purposes
  require their targets, and AEM ``RATE_FRAME`` placeholder rejection covers ``ref_frame_b``.
  CDM documents that unavailable XML leaves are removed before order/duplicate checks.
- Family records and the shared contract distinguish XSD structure/value domains from book
  conditional requirements. Missing frames, OPM maneuver mass, OEM interpolation degree, and
  overlapping OEM usable spans remain validation failures. OCM book defaults may remain omitted.
  Earlier combined ODM envelopes and attitude 1.0 remain outside the supported scope.
- APM and AEM 1.0 remain rejected by decision; the APM record documents the 1.0/2.0 differences
  and why 1.0 rates, ``B2A`` directions, inertia signs and spin conventions cannot be mapped
  without guessing. ``EULER_ROT_SEQ`` no longer reads ADM 1.0 numeric sequences such as ``313``
  in 2.0 messages (breaking); use the letter spelling (``ZXZ``).
- TDM 1.0 XML is read as input-only, like TDM 1.0 KVN, following the NDM/XML 1.0 schema
  (505.0-B-1). The bundled 1.0 schema files come from the archived SANA registry. 1.0 XML
  rejects time systems outside the schema's closed list, ``RHUMIDITY`` units and the qualified
  form; 1.0 KVN keeps the book's open ``TIME_SYSTEM``. Generation of a 1.0 message is refused.

Additional conformance corrections
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

- Restricted TDM 1.0 input is supported in KVN and XML; generation remains unsupported.
  TDM closed categories ignore case, and shared categorical lookup accepts blank/underscore runs.
- KVN treats standard-field ``N/A`` values as absent. In XML this convention is restricted
  to CDM; other families retain literal text and validate numeric values against their types.
  Comments, user-defined values, and CDM ``MANEUVERABLE=N/A`` retain their content.
  CDM XML refuses output whose N/A values would be lost on read-back.
- Quaternion validation accepts squared norms in [0.99, 1.01] and reports a field-named
  out-of-range error. CDM originators may contain blanks, including ``ESA SST``.
- OCM accepts omitted orbit-counter basis and SCLK parameters with the book's implicit defaults.
  ``USEABLE_START_TIME``, ``USEABLE_STOP_TIME``, and ``OEB_PARENT_FRAME_EPOCH`` now use
  Rust ``Epoch`` rather than ``CalendarEpoch`` (breaking). XML input omits the TC1 covariance
  linkage check that its schema cannot represent; general validation and KVN generation still enforce it, while XML generation permits schema-valid round trips.
- Python errors now share ``NdmError`` as their base; ``NdmFormatError`` and ``NdmEpochError``
  no longer derive from ``ValueError``, and ``NdmIoError`` no longer derives from ``OSError``
  (breaking). I/O errors retain file and errno attributes. ``AcmCovarianceMatrix`` now takes
  ``(cov_type, cov_lines, cov_basis=None, cov_ref_frame=None, comment=None)`` (breaking).
- APM, AEM, and ACM KVN generation checks quaternion magnitude after numeric rounding,
  including ACM maneuver targets, and refuses invalid rounded output before writing bytes.
- OCM validates each metadata time bound independently and rejects negative nominal drag
  coefficients while retaining the documented zero-drag exception.
- Shared vector readers reject nonzero numeric underflow instead of replacing components
  with zero; representable subnormal values remain accepted.
- Combined XML rejects forbidden XML characters throughout the document and accepts
  namespace-aware schema-location hints on envelope text fields.
- Conformance records clarify RDM's intentional international-designator compatibility,
  TDM/RDM time-window checks left to callers, and OCM numeric exceptions.

- CDM missing-value compatibility also applies to namespace-qualified XML. Empty optional
  leaves cannot conceal foreign, unbound, or mixed namespaces.

- KVN input preparation combines text checks with normalization detection. Integer output
  checks numeric bounds directly without text cleanup, and maneuver validation reuses the fast
  numeric parser.
- KVN numeric parsing and AEM row parsing avoid duplicate scans. TDM reuses per-keyword
  timestamp state, OCM maneuver output parses each numeric token once per write, and CDM
  normalization/preflight avoids unnecessary allocations and identical-output reparsing.
- History validation reuses timestamp comparison keys. KVN parsing avoids duplicate input
  normalization, and covariance output revalidates rounded matrices only when values change.
  Typed Python readers inspect the message header before parsing without a second full-file scan.

- TLE export requires Earth/TEME/UTC metadata and canonicalizes rounded cyclic-angle boundaries.
- OPM/OMM KVN output refuses empty or comment-only spacecraft blocks before writing.
- AEM compares adjacent usable bounds only within the same declared time system.
- CDM/TDM XML accepts schema-listed lowercase categories; TDM reads present-empty schema defaults.
- ACM XML enforces the attitude-determination method enumeration; OCM uses numeric revolution-basis
  equality and standard XML double spellings for time offsets.
- OMM recognized theory spellings share their conditional checks, without guessing the propagator
  behind ``SGP/SGP4``. Family records now state the remaining book/schema and storage boundaries.
- Shared KVN reading, in every family: CR, LF, CRLF, and LFCR line endings, blank lines anywhere,
  indented records (including the version and header), TABs as blanks, lines over 254 characters,
  a leading ``+`` on integers, and ``_`` for a blank in ``ROCKET BODY``. A blank or ``N/A`` optional
  value is absent; a blank mandatory value is missing at its record. Trailing junk after a number is "Invalid ... for KEY"
  at that line; non-ASCII errors name the byte column and code point. The edition is checked
  before the structure, so unsupported editions report ``parse.unsupported_input_version``.
  UTF-16/UTF-32 input is refused as "input must be UTF-8".
- Mixed-case open text values are accepted and preserved in every family; OEM no longer enforces
  the ODM 7.5.3 single-case rule. A missing ``<?xml ...?>`` declaration is accepted.
- KVN writers trim values, refuse empty or ``N/A`` standard-field values and integers above 2³¹−1, name the field
  in non-ASCII refusals, and round to 16 significant digits with stable output spellings.

- ACM histories accept absolute calendar tags as well as relative offsets; each block uses one
  form. XML Euler/rate field ordering is corrected. The unused ``cov_confidence``, ``ad_epoch``,
  and determination ``attitude_type`` fields are removed.
- OCM applies book defaults for omitted KVN time system and trajectory/covariance composition;
  open book values remain usable in KVN and are checked against XML enums on XML output.
  XML numeric history rows reject nonzero underflow and empty value lists. Trailing unattached
  KVN comments are refused, and equinoctial covariance checks omit the constant retrograde factor.
- CDM nested data comments and RDM data-level comments merge canonically for KVN conversion.
  Empty optional blocks normalize to absent; RDM comment-only blocks still fail explicitly.
- RDM removes unsupported uncertainty methods, preserves a lone covariance reference frame,
  and always writes required KVN units. Python ``RdmMetadata`` requires re-entry status explicitly.
- OPM/OMM 2.0 output enforces that edition's angle bounds and TLE requirements. OMM's mean-motion
  derivative may be omitted where its theory permits, but XML still enforces its required shape.
- APM KVN omits metadata delimiters and preserves quaternion-derivative units. AEM data validation
  takes no attitude-type argument; metadata/type matching is checked by the segment or message.
  The Python ``TdmPath`` class is removed; path properties use strings with arbitrary supported participant sequences.
- Error codes cover all error categories, including Python file and EOF errors. Typed Python
  readers identify a wrong message family before parsing its body. Unused ``requirement``
  diagnostics are removed; available paths and source lines are retained in diagnostics.
  The unused Rust ``ValidationError::EmptyValue`` variant is removed; empty mandatory values
  report ``validation.missing_required_field``.
  Validation locations remain aligned for LF, CR, CRLF, and LFCR input.
  Python classes now identify their module as ``ccsds_ndm``.

- Breaking Rust validation changes: helpers taking unused headers or metadata have been
  removed; use the ``Validate`` trait or complete-message validation. OMM's
  ``OmmData::validate_with_metadata`` and ``TleParameters::validate`` now take the edition;
  validate a complete OMM for edition-dependent rules instead of its body or segment alone.
- Breaking ACM history representation: Rust ``AttLine``/``CovLine`` store a lexical ``epoch``
  separately from their numeric ``values``. Python ``att_lines``/``cov_lines`` use
  ``(epoch_string, values)`` tuples. History basis fields use strings so book-defined free text
  can be retained.
- ACM XML histories reject nonzero numeric underflow instead of silently replacing it with
  zero. Typed Python readers retain the generic reader's parse-operation and encoding diagnostics.

Interoperability and preservation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

- Combined XML follows the bundled NDM/XML 4.0.0 constituent editions. OPM, OMM, and OEM
  version 2.0 remain supported as standalone XML; combining them in the 4.0.0 envelope is
  explicitly refused on input/output instead of producing schema-invalid XML.
- Numeric readers accept unambiguous decimal/exponent spellings, including ``.5``, ``5.``, ``1e3``,
  and extra precision; malformed values, overflow, and nonzero underflow are refused.
  Generation remains canonical.
- Closed KVN enums accept supported uppercase/lowercase spellings; TDM categories and shared
  YES/NO lookup also accept mixed case. Unknown closed values are rejected. Unknown object types no longer silently become
  ``OTHER``. Open registry/ICD strings retain their supplied spelling.
- Schema numeric facets now apply in all notations: GM, sensor frequency, and RDM day
  intervals must be positive. Explicit book/schema conflicts are recorded per family.
- Rust ``UserDefined`` and ``OcmUserDefined`` now expose ordered ``entries`` with comment and
  parameter variants. XML retains interleaving and exact names. KVN refuses comment placements
  or populated empty blocks it cannot preserve rather than moving or dropping information.
- Python ``UserDefined.entries`` is an ordered snapshot of comment strings and ``(name, value)``
  tuples. ``OcmUserDefined.entries`` is a live ordered list of comment strings and parameter
  objects. The OCM ``user_defined`` projection is a list snapshot containing live parameter
  objects; use ``entries`` for structural edits. Parameter dictionaries preserve insertion order;
  ordered entries also retain duplicate names that a dictionary cannot represent.
- Python covariance construction requires every covariance component instead of inventing zero
  values. AEM attitude type, OMM mean-element theory, and CDM ephemeris declarations are supplied
  explicitly. Generated enum annotations describe the actual PyO3 classes rather than claiming
  Python's ``enum.Enum`` interface.

Conformance review and compatibility changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

- ACM rejects duplicate attitude-history timestamps and non-increasing covariance
  timestamps. History tags retain their lexical spelling; numeric-value rounding cannot merge
  distinct time tags. OCM checks same-representation start/stop ordering and numeric
  relative bounds against their declared span, allowing a documented rounding
  tolerance. These checks also apply after model edits and before generation.
- ACM KVN accepts the book's ``RATE_TYPE=NONE`` as an alias for omitted rates;
  canonical output omits the keyword. XML still rejects ``NONE`` because the
  bundled schema excludes that spelling.
- CDM miss distance and relative speed must be nonnegative, following their
  book-defined norm meanings even though the schema uses unrestricted doubles.
  Exact vector-norm equality is not enforced: the published example itself uses
  scalar and vector values that do not agree exactly. This boundary is documented
  alongside the remaining physical-validation limits.
- Typed floating-point KVN fields now consistently use the existing CCSDS 16-digit
  formatter, including shared model blocks and TDM metadata/observations.
  CDM accepts computed values under the same rounding policy while preserving its nonnumeric
  content-loss checks; XML retains full double precision.
  KVN writes floating-point signed zero as ``0.0`` while readers remain permissive.
- OCM maneuver rows require exactly one token per declared column. Extra values
  are rejected before output rather than silently dropped, and embedded whitespace
  cannot change the column count. Numeric maneuver signed zero also writes as
  ``0.0``; text identifiers and XML row spellings are retained.
- Combined NDM decodes escaped text and CDATA, accepts empty envelope text
  elements, ignores processing instructions, and rejects nested markup and
  unsupported text-field attributes instead of changing or discarding them.
- OMM enforces required TLE choices at XML boundaries while retaining book-valid
  KVN omissions. RDM revalidates edited confidence percentages and finite altitudes.
- ADM fields enforce seconds-only units. AEM writes ``ANGVEL_FRAME`` and accepts
  ``RATE_FRAME`` as a compatibility alias; optional frame/sequence metadata stays
  optional. ACM checks declared data blocks, comparable time ranges, supplied sensor order,
  known maneuver
  purpose targets, and target quaternions; multiple maneuver targets remain
  available in KVN and are refused by the narrower XML choice.
- OCM known composition checks also apply to lowercase spellings, including
  covariance units and eigenvectors. KVN refuses covariance rounding that breaks
  positive definiteness. Maneuver columns and nonadjacent duplicate time tags
  are checked, and XML facets apply on input as well as output.
- TDM validates per-keyword observation chronology, integer Doppler counts, and
  indexed participant references. Comment canonicalization, compatibility aliases,
  the 64-byte epoch limit, and remaining supported-profile limits are documented.
- Every current message family now uses the same verification criteria and
  public workflow matrix. Family records cite the publication baseline,
  requirements, decisions, notation limits, and regression evidence. The
  conformance index links shared validation and preservation policy in the validation contract.
- OPM, APM, AEM, and ACM 1.0 input is rejected. Earlier acceptance reused
  newer-format grammars and did not establish those editions' requirements;
  ACM was introduced in ADM issue 2. OEM 1.0 keeps its explicitly restricted
  KVN reader, now checking increasing epochs and nonoverlapping segment spans.
- CDM metadata requires explicit ``covariance_method`` and ``maneuverable``;
  omitted values no longer imply calculated covariance or maneuver capability.
  Both objects must use the same frame and appear in OBJECT1/OBJECT2 order;
  the five restricted identifier fields enforce CDM section 5.2.9.
- OMM's inherent Rust validator now delegates to the complete root validator,
  closing an edition-check bypass in validation and KVN/TLE generation.
- APM root validation reaches all attitude blocks and maneuvers. ACM validates
  history widths against declared attitude/rate/covariance types, conditional
  Euler sequences, sensor noise counts, and edited numeric values. APM, AEM,
  and quaternion ACM histories share the documented quaternion magnitude gate.
- TDM phase counts preserve exact decimal strings through parsing, construction,
  generation and conversion. Python observation ``value`` returns ``str`` for
  phase counts and ``float`` otherwise; pass text when constructing an exact
  count. Rust phase-count variants hold ``String`` and ``value()`` returns
  ``Option<f64>`` (``None`` for phase counts); ``value_to_string()`` works for
  either representation. ``ANGLE_TYPE`` and ``REFERENCE_FRAME`` retain book-valid
  ICD/registry values in KVN and refuse values outside the XML enumeration at
  XML boundaries. ``DATA_TYPES`` validates the declared observation keywords.
- Generated Python annotations describe enum/string inputs and standalone
  combined constituents precisely. Generic Rust ``Message`` now implements
  ``Validate``. Installed wheel and source-distribution checks run the same
  Python behavioral suite as the checkout.

Python API consistency
~~~~~~~~~~~~~~~~~~~~~~

- CDM ``relative_state_vector`` is now a live child object. Nested edits and
  assigned child references reach validation and generation, as in the other
  model blocks. Assigning ``state_vector_numpy`` preserves the state's comments.
- ``CdmData.from_numpy`` retains supplied ``od_parameters`` and
  ``additional_parameters`` objects, matching the regular constructor's live
  reference behavior.
- Quaternion constructors reject incomplete derivative groups, and CDM relative
  metadata constructors reject position without velocity or velocity without
  position. These inputs now raise ``ValueError`` instead of silently losing
  the supplied values.
- Optional arguments can be omitted from ``AdditionalParameters``,
  ``QuaternionState``, ``EulerAngleState``, ``AngVelState``, ``SpinState``, and
  ``InertiaState`` constructors.
- All NDM exception classes expose diagnostic fields, with ``None`` when context
  is unavailable. Generated type annotations include these fields on the base
  exception classes, and subclasses inherit them. Wrong-family parsing errors
  identify the parse operation, expected family, and explicitly selected notation.
- The generated type annotations for generic ``from_str`` and ``from_file``
  return the union of supported message classes instead of ``Any``.

OCM current standard and numerical validation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

- OCM supports the June 2026 ODM technical corrigendum's contact metadata,
  covariance-to-trajectory identifiers, and ICD-defined user sequences in KVN.
  XML generation refuses these additions because NDM/XML 4.0.0 cannot express
  them; streaming output fails before emitting bytes.
- ``OcmData.user`` now uses ``OcmUserDefined`` with an ordered live list of
  ``OcmUserDefinedParameter`` objects. Sequence compositions retain their raw
  ``records``; ordinary message families keep the shared ``UserDefined`` type.
- Known OCM trajectory and covariance compositions enforce record dimensions
  and units. Covariance matrices enforce positive definiteness and their
  declared ordering, including covariance/correlation consistency. Malformed
  records that previously passed validation are now rejected.
- Published G15 and G19 examples are retained as negative fixtures; annotated
  positive adaptations correct G15's composition and G19's covariance linkage.

OEM producer compatibility
~~~~~~~~~~~~~~~~~~~~~~~~~~

- OEM 1.0 accepts comments between header and metadata assignments, between
  segments, and after the final record. Comments between ephemeris records
  remain invalid. Comment text is retained in the corresponding model block.
- OEM 2.0 and 3.0 accept header comments between header assignments. All OEM
  KVN readers accept comments between metadata assignments and unique header
  and metadata assignments in any order; duplicate keywords and a version line
  anywhere but first remain errors. OEM XML accepts unique header and metadata
  children in any order. XML comments must still form one contiguous group per
  header or metadata block; separated groups remain unsupported. OEM 1.0 accepts
  mixed-case book-listed text values.
- The shared XML reader accepts equivalent UTF-8 XML 1.0 declarations and
  omission of an unused schema-instance namespace. Used prefixes still need
  valid declarations; generated XML keeps the standard declaration and namespaces.
- OEM KVN ignores trailing ASCII padding, including on comments, and accepts overlong input
  records. Generated lines still fit the 254-character limit.
- OEM KVN reads comments between a covariance epoch or frame and its first
  numeric row into that matrix. XML and KVN preserve their associations on later matrices too;
  canonical KVN places each matrix's comments before its epoch.
- OEM KVN reads complete covariance matrices without section markers when an
  ``EPOCH`` follows the state records; generated KVN writes the standard markers.
- XML deserialization errors name a nearby source line and column.

Python threading and parsing fixes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

- Python parsing, validation, generation, conversion and file I/O now release
  the GIL while the Rust core runs, so other Python threads keep running.
  Building and reading Python objects still holds it.
- KVN version lines (``CCSDS_xxx_VERS``) and APM ``EPOCH`` reject a bracketed
  unit instead of silently dropping it, in every family.
- OEM segments whose XML TIME_SYSTEM differs only by surrounding whitespace,
  such as ``UTC`` padded with spaces, now count as one time system. The
  original text is kept.
- XML output writes unit-bearing values of very large or very small magnitude
  in exponent form (``1e+308`` instead of 309 digits); every value still reads
  back exactly.
- Passing a NumPy array with the wrong number of dimensions now raises
  ``ValueError`` naming the input and its shape, instead of
  ``TypeError: 'ndarray' object is not an instance of 'ndarray'``.
- A malformed KVN time tag is now a located KVN syntax error
  (``parse.kvn.syntax`` with line and column) in every family, like a malformed
  number, instead of an unlocated epoch error.
- OEM metadata errors report real field paths (for example
  ``metadata.start_time``) instead of names derived from the message text.
- OEM useable spans are checked for real overlap: segments no longer have to be
  in time order, and only complete spans are compared.
- OEM KVN rejects a second covariance ``EPOCH`` before its matrix, naming that
  line.
- OEM KVN generation no longer allocates per covariance matrix when
  ``COV_REF_FRAME`` is set.
- OEM XML output no longer writes ``units`` attributes on state and covariance
  values. The schema makes them optional and each value has a single fixed
  unit, so the output is smaller and parsing it back needs no per-value
  allocations. Input with ``units`` is still accepted and checked.
- Python: a wrong element type in a record list raises ``TypeError`` instead of
  ``ValueError`` (breaking). Reading a file of another family raises
  ``NdmUnsupportedMessageError`` naming the file instead of ``ValueError``
  (breaking). Epoch strings rejected by constructors and setters raise
  ``NdmEpochError``, a subclass of ``NdmError``. ``NdmIoError`` carries ``errno``
  and ``filename`` (and ``filename2`` for ``convert_file``).

Book-aligned OEM and XML parsing (breaking change)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

OEM parsing and validation now take their rules from CCSDS 502.0-B-3 and the
NDM/XML books and schemas rather than project policy; the remaining limits are
listed in the OEM conformance inventory. Newly accepted:

- KVN blank lines anywhere, trailing blanks after ``META_START`` and the
  covariance markers, empty optional values, and an empty covariance section.
- Overlapping total spans without useable bounds, object names differing only
  in case, covariance matrices with equal epochs, and MET/MRT durations (including
  ``REF_FRAME_EPOCH``, now a time-system-dependent ``Epoch``) such as
  ``0000-000T00:10:00``.
- XML namespace declarations and schema-location hints on any element of a
  standalone message (ODM 8.12.7 still limits combined constituents to ``id``
  and ``version``), the qualified (``ndm:``-prefixed) element form in every
  family, and ``INF``/``-INF``/``NaN`` OEM state and covariance values in XML.
- Mixed calendar and day-of-year epochs with different fractional precision now
  compare correctly in OEM, AEM and OCM.

Newly rejected:

- XML documents in every family with a non-UTF-8 encoding, or with an
  undeclared prefix used in a schema hint.
- OEM KVN covariance rows of the wrong width.
- Time tags with more than 16 fractional-second digits, in every family except
  TDM (ODM 7.5.10 and the matching ADM, CDM and RDM rules).
- Empty XML values for optional numbers, epochs and enumerations, except the documented
  CDM compatibility and schema defaults. CDM standard-field ``n/a`` is treated as absent;
  optional XML strings outside CDM retain empty text literally. A
  malformed optional value can no longer silently drop its ``units`` attribute.
- XML special values spelled other than ``INF``, ``-INF`` or ``NaN`` (for
  example ``inf`` or ``+INF``), in every family.
- OPM ``nil`` and ``xsi:nil`` attributes, which the OPM schema does not allow.

OEM review follow-up:

- Shared XML parsing rejects literal ``]]>`` in text, empty prefixed namespace
  bindings, duplicate schema-location attributes hidden behind prefix aliases,
  and empty or reserved ``xml`` processing-instruction targets.
- OEM 3.0 and OEM 2.0 are now **Verified** in the support matrix. OEM 2.0 is
  reviewed against 502.0-B-2 with Corrigendum 1 and its schema; it differs
  from 3.0 in the header, and two looser readings (text case, ``-0``/special
  values) are documented. Every shipped OEM example is checked as 2.0. ODM 5.2.4.7 (enough
  records for the interpolation method) is recorded as a requirement the book
  leaves undefined rather than a conformance gap.
- OEM 1.0 retains restricted input-only compatibility: it uses the shared model and
  cannot be written. XML is rejected for 1.0, a KVN-only format, and so is
  content that 502.0-B-1 does not have (``CLASSIFICATION``, ``MESSAGE_ID``,
  ``REF_FRAME_EPOCH``, accelerations, covariance). Some valid 1.0 files are
  still rejected; the OEM conformance page lists them.
- OEM covariance epochs are no longer required to fall inside the segment's
  ``START_TIME``/``STOP_TIME``: table 5-3 says they should, but the book's own
  example G-14 does not, so the shipped ``oem_g14.xml`` is now the example as
  published. State epochs are still bounded.
- OEM and AEM KVN numbers outside the double range (ODM 7.5.7e) are rejected
  instead of becoming infinity or zero.
- AEM KVN history numbers get the same reading as OEM: any number of digits
  and integer-form values of any size.
- OEM KVN reading accepts an unterminated last line and ephemeris and
  covariance numbers with more than 16 digits (such as the 17-digit shortest
  spelling of a double) or integers outside the 32-bit range. These break
  clear book rules but are common and lose nothing; writing still follows the
  book and rounds numbers to 16 significant digits.
- OEM KVN generation rejects an empty ``INTERPOLATION`` or ``COV_REF_FRAME``,
  which KVN would read back as absent; converting an XML ``<INTERPOLATION/>`` to KVN now fails
  instead of dropping it.
- Python: the NumPy inputs of OEM, AEM and CDM accept integer arrays and
  nested lists, converting them to float, instead of raising a confusing
  ``TypeError``.
- Python (breaking): ``OemData``, ``OemData.from_numpy``, ``OmmData``,
  ``CdmData``, ``CdmData.from_numpy``, ``CdmStateVector`` and
  ``StateVector`` take ``comment=`` instead of ``comments=``, matching their
  ``comment`` attribute and the other families. ``CombinedNdm`` keeps
  ``comments=``, the name of its attribute.
- An OEM KVN covariance section holding only comments is accepted; its
  comments join the data comments.
- An empty OEM KVN ``COV_REF_FRAME`` is absent, like other empty optional
  values.
- Mixed-case ``RSW``, ``RTN`` and ``TNW`` (ODM 3.2.4.11) are accepted and
  preserved as written in OEM KVN ``COV_REF_FRAME``.
- KVN syntax errors in every family carry their reason as the message instead
  of an empty message.
- Python: assigning an empty snapshot back to an empty ``OemData`` is a no-op
  instead of an error.
- Python: setting ``x_ddot``, ``y_ddot`` or ``z_ddot`` on a ``StateVectorAcc``
  keeps explicit XML units, like assigning ``state_vector_numpy``.
- Where the CCSDS books are unclear or conflict, parsing takes the permissive
  reading; OEM's remaining strict choices are listed in its conformance
  document.

Generated XML now declares ``xmlns:xsi`` and ``xmlns:ndm`` on the root and
writes special values as ``INF``, ``-INF`` and ``NaN``. KVN comments drop
trailing blanks, which the book makes insignificant.

Python docstrings now carry the CCSDS book reference of every field whose core
documentation has one.

Earlier OEM corrections in this release: KVN interpolation degrees accept a leading plus and must
fit a positive signed 32-bit integer; malformed optional OEM XML numbers and
epochs raise errors instead of being treated as absent; OEM optional strings and
shared ODM header strings retain literal content, including ``n/a``; shared XML
parsing rejects mixed container text, invalid character references and
malformed comments.

Python binding corrections (breaking change)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

``validate()`` on ``OemSegment``, ``OemMetadata``, ``OemData``, ``AemSegment``,
``AemMetadata``, ``AemData``, ``AcmSegment``, ``AcmMetadata`` and ``AcmData``
now raises ``NdmValidationError``, like message-level ``validate()``, instead of
``ValueError``. ``NdmValidationError`` is not a ``ValueError`` subclass.

``OemCovarianceMatrix(epoch, values, cov_ref_frame=None, comment=None)`` makes
the last two arguments optional. A wrong-typed element in a repeated field now
reports ``"<field>[<index>] must be <Type>"`` in every family.

``AemData.attitude_states_numpy`` assignment no longer partially writes when a
later element is invalid. The OEM NumPy setters update existing records in
place, keeping record identity, covariance frames and comments, and units, and
write nothing if any element is invalid. Covariance metadata supplied without
covariance epochs and values raises ``ValueError`` instead of being discarded.

Rust parsing controls (breaking change)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

``ParseOptions`` and the ``from_str_with_options``, ``from_file_with_options``,
``convert_with_options``, and ``convert_file_with_options`` functions have been
removed. Use ``from_str``/``from_file`` or their ``*_with_notation`` variants,
and use ``convert``/``convert_file`` for conversion. Applications that need
input-size or record-count limits should enforce them before parsing; XML depth
remains protected by the library's fixed safety limit.

Python metadata construction (breaking change)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Metadata constructors now require explicit interpretation fields instead of
silently supplying defaults:

- ``AemMetadata`` requires ``ref_frame_a``, ``ref_frame_b``, ``start_time``,
  ``stop_time``, and ``time_system``.
- ``OemMetadata`` requires ``center_name``, ``ref_frame``, and ``time_system``.
- ``OmmMetadata`` and ``OpmMetadata`` require ``center_name``, ``ref_frame``,
  and ``time_system``.
- ``RdmMetadata`` requires ``center_name`` and ``time_system``.
- ``AcmMetadata``, ``ApmMetadata``, ``OcmMetadata``, and ``TdmMetadata`` require
  ``time_system``.

Calls omitting these arguments now raise ``TypeError``. Pass the values appropriate
to your message explicitly; keyword arguments make the intended fields clear.

``interpolation_degree=0`` is now rejected instead of being treated as absent.
Use ``None`` to omit the degree, or a positive integer when specifying it.
See :doc:`guide/workflows` for metadata construction guidance.

``AemData`` and ``AemData.from_numpy`` now require an explicit
``attitude_type`` instead of inferring it from unambiguous row widths.
``OemData`` names its optional covariance collection ``covariance_matrices``;
the mutable model property remains ``covariance_matrix``.

Release follow-up corrections
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

- XML N/A handling no longer masks invalid non-CDM numeric values or their units.
- OCM XML with repeated covariance blocks round-trips in standalone and combined messages.
- TDM 1.0 input excludes later-edition fields according to the original 1.0 tables.
- KVN scalar preflights consistently normalize XML padding and internal tabs.
- Python wheels include the typing marker and exclude cached bytecode. NdmIoError stubs
  describe file attributes, and RdmSpacecraftParameters accepts every exposed field.
- Invalid UTF-8 file errors retain their I/O category and now identify the first invalid byte.
