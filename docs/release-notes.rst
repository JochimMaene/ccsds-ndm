Release notes
=============

Unreleased
----------

Additional conformance corrections
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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
- KVN writers trim values, refuse empty or ``N/A`` values and integers above 2³¹−1, name the field
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
  ``TdmPath`` is removed; path properties use strings with arbitrary supported participant sequences.
- Error codes cover all error categories, including Python file and EOF errors. Typed Python
  readers identify a wrong message family before parsing its body. Unused ``requirement``
  diagnostics are removed; available paths and source lines are retained in diagnostics.
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
- Closed KVN enums accept supported uppercase/lowercase spellings, but arbitrary mixed case
  and unknown closed values are rejected. Unknown object types no longer silently become
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
- OPM, APM, AEM, TDM, and ACM 1.0 input is rejected. Earlier acceptance reused
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
- OEM KVN ignores trailing ASCII spaces when checking the input line-length
  limit, including on comments. Generated lines still fit the 254-character limit.
- OEM KVN reads comments between a covariance epoch or frame and its first
  numeric row into that matrix. XML preserves them; KVN writing still rejects
  comments on later matrices when their association cannot be retained.
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
  ``NdmEpochError``, still a ``ValueError``. ``NdmIoError`` carries ``errno``
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
- Empty or ``n/a`` XML values for optional numbers, epochs and enumerations in
  every family, which were silently treated as absent; the schema types do not
  allow them. Optional XML strings keep ``n/a`` and empty text literally. A
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
- Mixed-case ``RSW``, ``RTN`` and ``TNW`` (ODM 3.2.4.11) are rejected in OEM
  KVN ``COV_REF_FRAME``.
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
