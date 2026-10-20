# OCM 3.0 Focused Conformance Evidence

This inventory records maintainer evidence for standalone OCM 3.0. The
[support matrix](../support-matrix.md) is the user-facing statement of current support.

## Scope and authorities

- Message edition: OCM 3.0.
- Semantic and KVN authority: CCSDS 502.0-B-3 plus editorial corrigendum 1 and technical corrigendum 1 (June 2026), principally sections
  6 and 7 and the OCM implementation-conformance statement in annex A.
- XML authority: NDM/XML schema set 4.0.0 and OCM schema 3.0. This schema predates the technical corrigendum; the explicit
  conversion refusals below apply.
- Profile: strict standalone parsing, self-contained validation, deterministic KVN/XML generation,
  and notation conversion through the shared Rust generation gate.

## Executable evidence

The `ocm` and `ocm_kvn_allocations` suites establish:

| Concern | Evidence |
| --- | --- |
| KVN lexical and marked-block structure | The OCM scanner rejects non-ASCII/control input, malformed or unknown assignments, duplicate and reordered keywords, unknown/nested/mismatched blocks, invalid logical-block order, and comments outside the beginning of a block. OCM lines may have arbitrary length under CCSDS 502.0-B-3 section 7.3.3. |
| XML structure | The shared XML sequence engine is registered for the complete OCM root, header, metadata, data, trajectory, physical, covariance, maneuver, perturbation, orbit-determination, and user-defined structures. It rejects unknown, duplicate, reordered, and non-schema nested content and attributes. |
| Silent-loss corrections | Complex `pert`, `od`, and `user` elements no longer use the scalar nullable adapter, so they survive XML parsing. KVN retains optional GM units. `DC_REF_DIR` and `DC_BODY_TRIGGER` use the schema's single three-number lexical value rather than generating nested Rust field elements. |
| Fixture and history preservation | The five current KVN fixtures and the shipped XML fixture retain their typed model under source-notation regeneration. KVN/XML crossings retain trajectory, physical, covariance, maneuver, perturbation, OD, and user blocks represented by the corpus. G19 is adapted for current covariance linkage and cannot cross to the older XML schema. Every generated XML document validates against the official 4.0.0 master schema. Schema validation runs through libxml2, which establishes structure, ordering, and lexical form; it is not evidence of numeric domain validity, because libxml2 accepts NaN against bounding facets (see the XSD oracle policy in the [validation contract](../design/validation-contract.md)). |
| Generation boundary | Materialized and streaming KVN generation reject non-ASCII free text while preserving OCM's arbitrary line lengths. Typed scalar, measure, fixed-vector, trajectory and covariance values use the shared 16-significant-digit formatter, rounding when needed and refusing values whose rounded spelling would overflow. `kvn_scalar_measures_wrappers_and_vectors_use_the_same_digit_limit` checks representative scalar/vector paths and zero-output refusal; the TIME_AND_ANGLE vector regression proves both notation paths. |
| Resource behaviour | Parsing 10 versus 1,000 trajectory and covariance records uses one owned numeric vector allocation per record without per-record reallocation. Maneuver parsing is bounded by its owned strings/vectors. Validated streaming generation has record-independent temporary allocation overhead for trajectory, covariance, and maneuver histories; materialized storage stays output-proportional. |
| Reproducible workloads | `ocm_{trajectory,covariance,maneuver}_{kvn,xml}_history_scaling` measure parse and generation of each history kind — CARTPV trajectory, CARTP covariance, relative-time maneuver — at 100, 10,000 and 50,000 records in KVN and the first two in XML, the sizes every history-carrying family shares. The shared family matrices add representative small-message comparisons. |
| Shared surfaces | `library::parsing` and `library::output`, Python API tests, and the shared generation plumbing exercise parsing, generation, structured diagnostics, and Rust-core delegation. |

The two shipped KVN comments that used a Greek eta were normalized to the ASCII word `eta`; strict
KVN processing does not silently accept or regenerate the non-ASCII spelling.

## Normative inventory reconciliation

Annex A, section A2.5.4 contains the OCM implementation-conformance statement. Its logical
inventory is reconciled as follows; the counts are statement rows, not public capability cells.

| ICS group | Rows | Reconciled implementation evidence |
| --- | ---: | --- |
| Header | 7 | Version, creation date, originator, message ID, and leading comments are covered by the common ODM header model and strict root/header sequence checks. |
| Metadata | 51 | Required identity/time fields, SCLK dependencies, units, ordering, duplicates, and comments are covered by typed metadata validation plus KVN/XML structural checks. |
| Trajectory | 23 | Block metadata, composition/units cardinality, time tags, epoch ordering, fixed record shape, and representability are covered by the trajectory model and history tests. `validate_ocm_line_values` additionally rejects non-finite numbers in every `trajLine`, naming the failing line and column. |
| Physical | 54 | The complete optional scalar/vector XML/KVN model is registered; required-field, unit, order, duplicate, and notation-preservation behavior is exercised by the fixture corpus. `OcmPhysicalDescription::validate` covers the block's own editable values: `DRAG_COEFF_NOM`, the eleven areas, three masses, two percentages, `REFLECTANCE`, the three attitude angles, and the remaining plain doubles for finiteness. |
| Covariance | 18 | Block metadata, ordering, epoch sequence, composition/units, fixed record shape, and representability are covered by covariance validation and history tests, and `validate_ocm_line_values` rejects non-finite numbers in every `covLine`. |
| Maneuver | 35 | Time-tag choice, composition/units, duty-cycle dependencies (including `TIME_AND_ANGLE` vectors), epoch sequence, and representability are covered by typed validation and regression tests. |
| Perturbation | 33 | The complete nested block is retained rather than passed through the former nullable-scalar adapter; schema order, legal attributes, units, and round trips are exercised. `OcmPerturbations::validate` covers `GM`, `OBLATE_FLATTENING`, `ALBEDO_GRID_SIZE`, and the finiteness of `EQUATORIAL_RADIUS`, `CENTRAL_BODY_ROTATION`, and the geomagnetic and solar-flux values. |
| Orbit determination | 33 | The complete nested block is retained with typed required/optional values, legal units, schema order, and fixture round trips. `OcmOdParameters::validate` covers the positive-integer counts, `GDOP` and `WEIGHTED_RMS` (including NaN and both infinities), the OD spans, the eigenvalue lengths, `OD_CONFIDENCE`, and `SEDR`. |
| User defined | 5 | Marked-block placement, comment placement, parameter names/values, and XML/KVN preservation are covered. |

These OCM values are deliberately **not** validated, each for a recorded reason rather than by
omission:

- `DRAG_COEFF_NOM` accepts zero because ODM says it disables atmospheric drag, while the XSD's
  `positiveDouble` excludes it. KVN preserves zero; XML input and generation require a positive value.
- `DAYS_SINCE_FIRST_OBS`, `DAYS_SINCE_LAST_OBS`, `DC_PA_START_ANGLE`, and `DC_PA_STOP_ANGLE` are
  validated for finiteness only, because ODM's domain is wider than the 3.0 schema's. The
  book-valid value is preserved and KVN writes it; XML generation refuses the conversion with the
  offending field path rather than altering the value. Both accepted boundaries generate
  schema-valid XML.
- `OEB_Q1`–`OEB_QC` are not checked for unit norm, because ODM assigns `-999` as the
  tumbling-object flag, so no norm rule exists.
- `OEB_MAX`/`OEB_INT`/`OEB_MIN` ordering — the book's wording is descriptive, not a `shall`.
- `MAN_*` line values remain `Vec<String>` because the columns are heterogeneous, but every
  row must contain exactly the columns declared by `MAN_COMPOSITION`, with one nonempty
  whitespace-free token per column (ODM §§6.2.8.14–18 and 8.11.18). Extra columns are rejected
  rather than dropped during generation; embedded whitespace cannot split one value into several
  columns. Every declared-numeric column must hold a finite number. `ACC_INTERP`,
  `THR_INTERP`, and `DEPLOY_ID` are the only non-numeric columns in ODM tables 6-8 and 6-9. No
  per-column domain is imposed, matching Orekit's `ManeuverFieldType`. The XSD types the line as a
  `xsd:string`, so the schema oracle cannot establish this rule and a direct test does.

  KVN generation preserves legal numeric spellings in those rows, but rounds numeric mantissas
  exceeding 16 digits through the shared formatter to satisfy ODM §7.5.7. Numeric signed zero
  follows the shared KVN normalization to `0.0`; text columns retain their spelling. It never rounds a
  text identifier such as `DEPLOY_ID`, even when it contains only digits. XML `manLine` is
  `xsd:string` and retains the original text. The `maneuver_numeric_precision_keeps_text_identifiers_and_legal_spellings`
  regression covers both composition spellings, text identifiers, rounding, and zero-output refusal
  of values whose rounded spelling overflows or whose nonzero text underflows a double.

Maneuver compositions select distinct columns, in order, from either ODM table 6-8
(propulsive) or table 6-9 (deployment); columns unique to the two tables cannot be mixed
(§§6.2.8.15–16). Reading retains the existing unambiguous comma- or whitespace-separated
compatibility spellings. A composition may select only its time tag: neither table requires
another column, and both parsers retain those empty-value records. Text deployment IDs remain
single tokens and may contain `=`, which does not turn a numerical history record into a KVN
assignment. All epochs in one maneuver block must be unique (§6.2.2.4), including repeated
nonadjacent epochs with equivalent decimal or calendar spellings. Maneuver histories need not
be monotonic; validation preserves their supplied order and compares retained epochs without
rounding through `f64`.

`INTERPOLATION_DEGREE` illustrates a book/schema distinction. Table 6-4 calls it an integer,
lists default 3, and conditionally requires it for interpolation other than PROPAGATE; the
XML schema makes it optional but positive when supplied. The library accepts omission as use
of the documented default instead of adding a required field or synthesizing a value. A supplied
zero remains book-readable in KVN but is refused on XML input and output. No interpolation
algorithm is executed, and record sufficiency for an externally chosen algorithm is caller context.

`ORB_REVNUM` must be finite and nonnegative. Table 6-4 calls the count an integer, but its
`ORB_REVNUM_BASIS` description refers to fractional counts `0.XXXX` and `1.XXXX`, and the XML
schema uses `nonNegativeDouble`. The library therefore retains nonnegative fractional counts
instead of truncating or rejecting them. `orbit_revolution_counts_are_finite_nonnegative_and_may_be_fractional`
and `xml_only_facets_are_checked_on_read_and_generation` exercise these decisions.

OCM's exception to the shared policy on [externally governed values](../design/validation-contract.md#registry-governed-values) is that its `OBJECT_TYPE` is delegated to the living
SANA Object Types registry, so the token is preserved losslessly and checked for offline syntax
rather than current membership — unlike the closed CDM and RDM lists. This mapping does not claim
byte-preserving regeneration.

### Metadata time bounds

ODM table 6-3 defines `TIME_SPAN` as `(STOP_TIME - START_TIME)` in days. Section 6.2.2.3
defines numeric relative tags as SI seconds from the common `EPOCH_TZERO`, including when
`TIME_SYSTEM` is SCLK. When both metadata bounds use the same representation (both numeric
or both calendar/ordinal), validation requires `STOP_TIME >= START_TIME`; equal bounds are
allowed. Ordering uses retained decimal tags or calendar fields without rounding to doubles.
When both bounds are numeric and `TIME_SPAN` is present, validation additionally checks
their difference divided by 86400.

The ordering rule follows the table's earliest/latest definitions. In
`ndmxml-4.0.0-ocm-3.0.xsd`, both bounds are optional `epochType` elements and the optional
span is `dayIntervalTypeUO`; the common schema defines its nonnegative value and optional
`d` units but expresses no relationship between these fields. The cross-field checks
therefore come from the book, rather than an XSD assertion.

Table 6-3 contains a SCLK anchor ambiguity: the `TIME_SYSTEM` row calls `EPOCH_TZERO`
the spacecraft clock epoch, while the `EPOCH_TZERO` row explicitly places it in UTC.
The library preserves that supplied anchor, requires the two SCLK clock parameters,
and performs no clock conversion. Subtracting numeric relative tags does not interpret
the anchor and follows section 6.2.2.3's explicit SI-second definition.

The span comparison allows an absolute error of `max(1e-6, 8 * f64::EPSILON *
max(abs(START_TIME), abs(STOP_TIME)) / 86400)` days. The `1e-6` day allowance (86.4
milliseconds) accepts spans rounded to six decimal days; the second term accommodates floating-point
subtraction of large relative tags. This is an explicit implementation tolerance, not a
book-defined precision requirement. Values are preserved rather than repaired.

Missing bounds are not inferred from histories, and no span relation is checked if either
bound is absent or calendar-valued. Calendar duration subtraction and mixed-representation
comparisons require time-scale, leap-second, or clock interpretation outside this numeric
check. Calendar ordering needs no such conversion because all absolute tags share the
metadata `TIME_SYSTEM`. A supplied
`TIME_SPAN` must still be finite and nonnegative. These limits mean successful validation
does not certify that the metadata bounds describe the actual histories.

`epochs::relative_metadata_bounds_and_span_are_checked_in_all_routes` covers input,
mutation, both output notations, and zero-byte streaming failure.
`epochs::relative_metadata_span_allows_rounding_and_optional_or_external_context` covers
rounded values, comparison boundaries, SCLK, and the deliberately unchecked cases.
`epochs::calendar_metadata_bounds_are_ordered_without_subtracting_time_scales` covers
equal and reversed calendar bounds through parsing, mutation, and generation.

### Trajectory usable bounds

The trajectory `USEABLE_START_TIME` and `USEABLE_STOP_TIME` fields currently accept calendar
or ordinal epochs only, even when trajectory rows use relative tags. The common `EPOCH_TZERO`
anchors relative rows; the library does not convert those tags into the bounds' representation
or prove that the usable interval covers the records. This is an explicit supported-profile
restriction: ODM §6.2.2.3 broadly permits relative time tags and the XML fields use `epochType`,
but table 6-4 specifically refers these bounds to §7.5.10's calendar syntax. Supporting relative
usable bounds would require changing the public field types; the present release keeps the
calendar interpretation and rejects relative bounds rather than guessing a conversion.

## Numerical record contracts

ODM sections 6.2.5.11 and 6.2.7.10 and the [SANA Orbital Elements](https://sanaregistry.org/r/orbital_elements/)
and [Orbital Covariance Matrix Types](https://sanaregistry.org/r/orbital_covariance_matrix_types/)
registries govern the numerical compositions. The registry definitions were reviewed on
2026-10-15. The implementation knows all 15 trajectory compositions and all 17 additional
covariance representations present on that date. Known names
are recognized case-insensitively while preserving the supplied token. Registry extensions with
unrecognized names remain opaque: their numbers are finite and their epochs are checked, but their dimensional,
unit, and physical interpretation cannot be established without a registry definition.

- Known trajectory records have exactly the declared number of elements; known `TRAJ_UNITS`
  lists have the corresponding cardinality and standard units.
- Known covariance matrix records have `N(N+1)/2` entries for LTM/UTM and `N²` for FULL,
  LTMWCC, and UTMWCC. Covariance `COV_UNITS` describes standard deviations, not squared
  covariance entries, and must list all diagonal components with their standard units.
- Matrix entries are reconstructed in their declared ordering. FULL must be symmetric.
  Positive definiteness is checked by Cholesky factorization after scaling the matrix to
  correlation units; positive diagonal entries alone are insufficient. The bounded stack
  workspace handles the largest known composition, the 10-dimensional TCARTPVA.
  Before KVN output, the same record constraints are checked on the actual rounded 16-digit
  numbers, for both matrix and eigenvector representations. If rounding would make a matrix
  singular, indefinite, or inconsistent, or move an eigenvector basis outside its norm tolerance,
  generation refuses the conversion before writing bytes; XML can still retain the original
  doubles. No matrix repair or extra comparison tolerance is applied.
- Hybrid covariance/correlation orderings must agree to absolute correlation tolerance `1e-12`.
  The book specifies no numeric comparison tolerance; `1e-12` is the implementation choice for
  double-precision comparison. It does not repair inconsistent values.
- `SIG3EIGVEC3` and `TSIG3EIGVEC3` have 12 and 13 entries, respectively, positive dispersions,
  and an orthonormal eigenvector basis, checked with the same explicit absolute dot-product
  tolerance `1e-12`. Lower-precision rounded bases outside that tolerance are refused.
  Their optional `COV_UNITS` lists describe diagonal standard deviations: `[km,km,km]`
  for SIG3EIGVEC3 and `[s,km,km,km]` for TSIG3EIGVEC3. Table 6-6 first says “each element”
  but then specifically requires diagonal standard-deviation entries. The latter definition
  governs this interpretation; the nine dimensionless eigenvector components do not add
  diagonal-unit entries. The schema's free-text `COV_UNITS` cannot establish this rule.
- `COV_SCALE_MIN` and `COV_SCALE_MAX` must be finite and `COV_CONFIDENCE` must satisfy its
  percentage domain. No undocumented relationship between scale factors is inferred.

`corrigendum::covariance_dimensions_orderings_and_positive_definiteness`,
`covariance_reconstructs_triangle_directions_and_checks_correlations`, and
`covariance_eigenvectors_units_and_scalar_domains` establish these contracts through edited
models and parsing/generation. Synthetic controls use complete, positive-definite matrices.

The published G15 example labels nine state elements as `CARTPV`, which specifies six.
Its original bytes remain in `data/invalid/ocm_g15_original.kvn`; the current positive fixture
is annotated and uses `CARTPVA`. The implementation follows the declared composition rather
than inferring acceleration from extra columns or accepting the inconsistent original example.

The published G17 propulsive composition spells `THR_ISP` as `ISP`, which is not a
selectable column in normative table 6-8. Its propulsive `MAN_UNITS` also adds an extra `n/a`
for the time tag, although table 6-7 defines these units for elements following that tag.
The original bytes remain in `data/invalid/ocm_g17_original.kvn`; the annotated positive
fixture corrects the keyword and removes that extra unit. This follows the declared standard
columns instead of inventing an alias for the example's typo. The regression
`published_g17_typo_is_preserved_separately_from_the_corrected_fixture` rejects the original
composition and round-trips the corrected input in both notations.

## June 2026 technical corrigendum

The primary authority is [CCSDS 502.0-B-3 Technical Corrigendum 1, June 2026](https://ccsds.org/wp-content/uploads/gravity_forms/5-448e85c647331d9cbaf66c096458bdd5/2026/10/502x0b3e2tc1_tc2603.pdf).
Its later pages mistakenly carry headers naming a different standard; the title and the actual
sections modified identify ODM section 6.2. The original annex A row counts above describe the
pre-corrigendum inventory, not the number of current keywords.

| Change | Implementation and decision | Evidence |
| --- | --- | --- |
| Contact metadata | Ten new optional free-text fields are retained: `OPERATOR_URL`, `OPERATOR_EMAIL`, `OPERATOR_PHONE`, `OPERATOR_AUTOMATED_URL`, `EMERGENCY_EMAIL`, `EMERGENCY_PHONE`, `EMERGENCY_AVAILABILITY`, `SSA_EMAIL`, `SSA_PHONE`, `OWNER_URL`. `COUNTRY` already existed. No email, URL, phone, or country-registry truth is inferred or checked. | `corrigendum::contact_keywords_round_trip_and_refuse_old_xml_without_output` checks every new keyword. |
| Covariance/trajectory linkage | Each trajectory must have a distinct nonempty `TRAJ_ID` when multiple trajectories and covariance histories coexist. Each covariance must identify its corresponding trajectory when there are multiple trajectory or covariance blocks. Any supplied covariance `TRAJ_ID` must resolve to a trajectory in this OCM; an identifier that could refer to several trajectories is rejected. | `corrigendum::covariance_links_are_required_resolved_and_unambiguous` tests missing, unknown, duplicate, and valid links. |
| User-defined sequences | `OcmUserDefined` retains ordered `OcmUserDefinedParameter` entries. A `COMPOSITION_<ID>` parameter owns its following raw `records`, terminated by the next user-defined assignment or `USER_STOP`. Names, comma-delimited composition, rows, and entry order survive KVN regeneration. Records cannot contain newlines, actual user-defined assignment delimiters, `USER_STOP`, or nonprintable characters; other ICD-defined punctuation and marker-like text is preserved. Leading/trailing whitespace and blank separator lines are normalized. | `corrigendum::sequences_preserve_rows_and_parameter_order_and_refuse_xml` and `sequences_reject_missing_id_rows_and_composition_columns`. |
| ICD boundary | The accompanying ICD defines column interpretation, time/event/sample ordering, and what each value means. Records stay opaque rather than guessing an epoch or numeric format. The library validates the message envelope and preserves producer row order; it does not claim to establish monotonicity under an absent ICD. | The typed model exposes rows without numerical reinterpretation. |
| Covariance dimensions | `TCARTPV` includes uncertain event time and has dimension seven; `CARTPV` has dimension six. The corrigendum clarifies their meaning without changing the token or representation. | `corrigendum::covariance_dimensions_orderings_and_positive_definiteness` establishes both dimensions. |
| XML representability | NDM/XML 4.0.0 cannot represent the ten new contact fields, covariance `TRAJ_ID`, or sequence rows. Their presence causes materialized and streaming XML generation to fail before output. XML input containing these new elements is rejected by the schema-based structural gate. They are never silently omitted or emitted as invented XML elements. | The contact, linkage, and sequence regressions above exercise these boundaries. |

The published pre-corrigendum G19 example has multiple covariance histories without trajectory
linkage. Its original bytes are retained in `data/invalid/ocm_g19_pre_corrigendum.kvn` and are
explicitly rejected under the current semantic baseline. `data/kvn/ocm_g19.kvn` is an annotated
adaptation adding the required IDs; it is no longer cited as an unchanged official example.
A single trajectory and single covariance need no IDs and remain XML-representable. A message
with multiple covariance blocks cannot satisfy both the current linkage rule and the older XML
schema; this is a published-edition conflict, not a reason to weaken semantic validation.

## Public-surface and artifact evidence

`bindings/python/tests/test_ocm.py`, `test_api_consistency.py`, and
`test_parse_and_generation_options.py` exercise the Python class, typed nested setters, generic
dispatch, strict parsing/generation options, and shared limits. The repository-wide
`package-python` and `package-rust` gates verify the built artifacts; they are release gates shared
by all family cells.

## Verification outcome

OCM 3.0 meets the common [verification criteria](README.md#verification-criteria) against
CCSDS 502.0-B-3 through the June 2026 technical corrigendum, on Rust and Python. The numerical
record review, current-corrigendum regressions, family corpus, shared public workflows, and
installed-artifact tests establish the same scoped evidence required of every current family.
The book/schema conflicts and externally governed interpretations above remain explicit limits
of that claim, not promises of unrestricted XML conversion or physical-model certification.
