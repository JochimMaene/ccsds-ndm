# OPM 3.0 and 2.0 conformance

This inventory covers the core CCSDS OPM 3.0 behavior: strict KVN and XML parsing, typed-model
validation, KVN and XML generation, conversion in both notation directions, and the Python
adapter over them. The normative sources are CCSDS 502.0-B-3 with Editorial Corrigendum 1 and the
NDM/XML 4.0.0 schema set with OPM schema 3.0. The XSD is primary for XML structure, lexical
types, and value facets; the book supplies semantics the schema does not express. KVN has no
schema artifact, so its wire requirements come from ODM sections 3.2 and 7.3–7.9.

Checks that need information outside a self-contained OPM — an exchange-partner ICD, registry
membership, or the physical definition of a reference frame or time system — are caller context.
They are not missing self-contained behavior and are not guessed; see
[externally governed values](../design/validation-contract.md#registry-governed-values).

## Requirement map

| Area | Normative source | Implemented behavior | Executable evidence |
| --- | --- | --- | --- |
| Message identity and structure | ODM 3.1.5, 3.2.1, 8.2–8.3 | One OPM root describing one object, ordered header/metadata/data, an XML 1.0 UTF-8 declaration with the shared producer-compatible spellings, `id="CCSDS_OPM_VERS"`/`version="3.0"` root attributes, and the supported-edition generation gate | `opm::parsing`, `opm::generation`, `library::xml` |
| KVN lexical and record structure | ODM 7.3.2–7.3.4, 7.4.3–7.4.7 | Printable ASCII (TAB reads as a blank), 254-character lines on output (longer lines are read), all four normative terminators, one uppercase assignment per line, and the `USER_DEFINED_` suffix repertoire accepted per table 3-3 rather than Annex F's narrower informative regex | `opm::parsing`, `opm::generation` |
| KVN keyword order and blocks | ODM 3.2.2–3.2.4, 7.4.8 | Fixed keyword order with unknown, duplicate, reordered, trailing, and incomplete content rejected; `TRUE_ANOMALY`/`MEAN_ANOMALY` share an ordering rank, so either fills the anomaly slot while a repeat is still a duplicate and both present is a choice violation | `opm::parsing`, `opm::generation` |
| XML structure | ODM 8.4–8.7; `ndmxml-4.0.0-opm-3.0.xsd` and common schema | Exact root and attributes, ordered known elements, no DTD or trailing document, fixed nesting limit of 16, and `units`/`parameter` attributes permitted only where the schema declares them | `opm::parsing` |
| Time semantics | ODM 3.2.3.2, 7.5.10–7.5.11 | `CREATION_DATE`, `REF_FRAME_EPOCH`, state-vector `EPOCH`, and `MAN_EPOCH_IGNITION` require the calendar or ordinal spelling, rejecting the XSD union's numeric branch, impossible fields, empty fractions, non-`YYYY` years, timezone offsets, and tokens past the 64-byte epoch bound | `opm::parsing`, `opm::validation` |
| Typed values, units, and ranges | ODM 3.2.4, 7.5.1–7.7.1; tables 3-1 to 3-3 | Required content, finite numbers, XSD facet ranges, exactly one anomaly, complete lower-triangular covariance, complete maneuver blocks requiring spacecraft mass, and closed unit enums matching the XSD enumerations | `opm::validation`, `opm::generation` |
| Comments | ODM 7.8.3–7.8.7 | Comments only at logical-block starts, preserved relative to surrounding assignment keys; the header parser assigns only post-version comments to the header; comment-internal whitespace is retained under 7.8.5 | `opm::parsing`, `opm::generation` |
| KVN generation | ODM 3.2, 7.3–7.9 | Deterministic ordered output identical across typed, versioned, type-erased, streaming, and file entry points; the shortest numeric spelling rounded to the 16-digit ODM limit; every generated Annex G line re-checked for printable ASCII and the 254-character bound | `opm::generation`, `opm_kvn_allocations` |
| XML generation | OPM 3.0 XSD in NDM/XML 4.0.0 | Deterministic validated XML; all five shipped OPM fixtures generate output accepted by the official master schema; every schema-constrained public field has focused mutation evidence | `opm::generation`, `opm_xml_allocations` |
| Conversion | ODM 3 and project semantic-preservation policy | KVN↔XML preserves the complete typed model and edition across every shipped fixture, including units, repeated maneuvers, covariance, and user-defined parameters; XML values beyond KVN's 16-digit limit round to a conforming spelling | `opm::conversion` |
| Diagnostics | Project diagnostic contract | Failures carry operation, notation, message kind, edition, stable code, model path, and — for KVN — byte/line/column with a 128-character token excerpt | `opm::parsing`, `opm::validation`, `library::errors` |
| Resource behavior | Project conformance policy | Atomic file replacement leaving no temporary file, panic-free streaming through failing sinks, and stack-resident numeric formatting without a per-value `String` | `library::output`, `opm_kvn_allocations`, `opm_xml_allocations` |
| Python adapter | Project binding contract | Strict parsing, validated generation, both conversion directions, file behavior, and structured exceptions delegate to the Rust core; `just audit` and `just stubs-check` enforce Rust-field exposure and stub agreement; `just package-python` exercises the installed wheel | `test_opm.py`, `test_parse_and_generation_options.py`, `test_api_consistency.py` |
| Performance | Project performance contract | `kvn_message_matrix/{parse,generate}/opm` and `xml_message_matrix/{parse,generate}/opm` measure the richest Annex G fixture with model construction outside the loop; generation includes the validation walk. Allocation budgets are the deterministic gate; timing stays informational | `cargo bench -p ccsds-ndm --bench kvn_benches -- kvn_message_matrix`, `opm_kvn_allocations`, `opm_xml_allocations` |
| Fuzzing | Project maturity policy | The generic KVN and XML fuzz targets reach OPM from checked-in minimal seeds and complete the reproducible smoke run without a crash | `just fuzz-all` |

## Deliberate boundaries

- **Comment position across notations.** The XSD gives `opmData` and `stateVectorType` separate
  `COMMENT` positions; the KVN keyword table has one slot ahead of `EPOCH`. KVN parsing assigns
  every pre-`EPOCH` comment to `data.comment`; KVN generation writes `data.comment` then
  `state_vector.comment`. An XML model with both populated therefore merges into the data section
  across a KVN hop — nothing is lost, reordered, or duplicated, and the merge is idempotent. Both
  model fields are kept because XML→XML preserves them exactly.
- **Reference frames stay open strings.** `REF_FRAME`, `COV_REF_FRAME`, and `MAN_REF_FRAME` are
  unrestricted `xsd:string` in the schema, and 3.2.3.3/3.2.4.11 say values *should* come from the
  named sets while permitting ICD-defined alternatives. No enum is imposed; only blank required
  values are rejected, which is model validation rather than an XSD enumeration rule.
- **`MAN_DURATION` rejects days.** The shared `TimeUnits` enum exposes `Day`, but the XSD's
  `durationType` allows only `s`. OPM maneuver validation rejects `TimeUnits::Day` before
  generation so a mutated model cannot emit schema-invalid XML; the shared type is unchanged for
  other families.
- **GM unit spelling is canonicalized in KVN.** ODM 7.7.1 admits only the table spelling, so the
  uppercase `KM**3/S**2` that the XSD also permits is written as `km**3/s**2` rather than failing
  generation. XML output keeps the spelling held by the model. KVN reading accepts that
  uppercase alias (`[KM**3/S**2]`); no other unit alias is accepted.
- **User-defined entry order.** `UserDefined.entries` retains XML comments and parameters in
  their original order, including interleaving. XML regeneration preserves that order. KVN
  requires leading comments, so conversion refuses an interleaved model before output. Parameter
  names are exact suffixes: a stored `USER_DEFINED_FOO` generates `USER_DEFINED_USER_DEFINED_FOO`.
- **GM must be positive in every notation.** The book does not state a range; `gmType` requires
  a positive value, and under the shared
  [facet rule](../design/validation-contract.md#authority-and-representability) that bound
  applies to KVN, the model, and XML alike so a valid model always converts.
- **Empty mandatory XML strings are rejected** as empty values (book over XSD), per the
  [shared rule](../design/validation-contract.md#authority-and-representability).

- **Zero delta mass is accepted.** 3.2.4.7 describes delta mass as negative, but the XSD's
  `deltamassTypeZ` permits zero; the schema governs the wire form.
- **`nil` and `xsi:nil` are rejected.** The OPM schema declares no nillable element, so an
  optional value is either present with a valid value or omitted.
- **XML source locations are omitted** where `quick-xml` does not reliably expose them, rather
  than invented.
- **Caller context is never guessed.** MET/MRT reference events, ICD-defined time systems and
  frames, whether a `:60` value coincides with a real leap second, registry membership, and the
  factual correctness of a supplied epoch all require information the message does not carry.

## Verification outcome

OPM 3.0 is verified on the Rust and Python surfaces: strict parsing, self-contained validation,
both generation notations, conversion, diagnostics, allocation budgets, and packaged artifacts
have received message-level review, and no known requirement gap remains. "Verified" means that
review, not exhaustive mutation of every editable value. OPM 2.0 has also received the edition-specific review below and meets the same criteria.

## Edition 2.0 reconciliation

The edition-specific authority is `data/odm_502x0b2s.pdf` and the official
`ndmxml-2.0.0-opm-2.0.xsd` with its common schema. The 3.0 inventory above is shared only
where those sources agree. ODM 3.0 annex J and a comparison of the header, metadata,
state, Keplerian, spacecraft, covariance, maneuver, and user-defined schema sequences
identify these model-relevant differences:

| Difference | Decision and evidence |
| --- | --- |
| `CLASSIFICATION` and `MESSAGE_ID` are not in the 2.0 header | Reject their presence in both parsed notations and mutated models; `conformance::odm2::opm_and_omm_2_reject_new_header_fields_in_both_notations`. |
| 2.0 `angleRange` is [-180, 360), while 3.0's is [-360, 360) | `RA_OF_ASC_NODE`, `ARG_OF_PERICENTER`, `TRUE_ANOMALY`, and `MEAN_ANOMALY` use the 2.0 range for 2.0 in every notation, so a valid 2.0 model always converts; `conformance::odm2::odm_2_angles_use_the_2_0_range`. |
| Maneuver delta mass uses 2.0 `negativeDouble`, while 3.0 uses `deltamassTypeZ` | Zero remains accepted only for 3.0. The 2.0 root gate rejects zero, including KVN/XML input and output; `conformance::odm2::opm_2_requires_negative_maneuver_delta_mass_in_every_route`. |
| A present XML spacecraft block requires all five spacecraft fields | The 2.0 common schema makes these fields required; 3.0 makes them optional. Partial blocks remain valid KVN/model content, but 2.0 XML input/output refuses them. `conformance::odm2::odm_2_partial_spacecraft_is_kvn_valid_but_not_xml_representable`. |
| Other logical blocks retain their typed structure | Every shipped OPM KVN fixture, with only its new header removed and edition changed, passes 2.0 KVN/XML preservation and the official 2.0 XSD; `conformance::odm2::opm_2_generation_is_schema_valid_and_round_trips`. |
| Public adapters retain edition | Python's shared conformance matrix covers both notations and all string/file/conversion routes separately for 2.0 and 3.0. |

**OPM 1.0 is not supported.** Its actual required-block and field rules differ from the
later grammar: ODM 502.0-B-1 tables 3-1/3-3 and §3.3 require edition-specific spacecraft
content and do not introduce the newer header. Relabelling a 3.0 fixture had accepted
later-only fields and omissions instead of implementing those rules. Input 1.0 is now
rejected until its own grammar and semantics are implemented and reviewed.

## Reproduction

Run `just verify` for the full quality checks plus the packaged-artifact gates.

Shared Rust/Python route coverage, including `test_conformance_matrix.py`, and installed
artifact checks are indexed once in the [common evidence table](README.md#shared-evidence).

An XML `userDefinedParameters` block with no parameters may be retained and regenerated as XML.
KVN generation refuses that empty or comment-only block: without delimiters, KVN cannot preserve
its presence or distinguish those comments from the previous logical block.

## Empty spacecraft blocks and elapsed-time profile

The 3.0 XSD permits empty or comment-only `spacecraftParameters`. These remain available
in XML; KVN generation refuses them because a flattened block with no spacecraft value
cannot preserve its presence or comment association. It neither drops the block nor
moves its comments. The shared regression
`conformance::generation::empty_spacecraft_blocks_are_xml_only_and_kvn_refuses_before_output`
checks OPM and OMM, including zero-byte streaming refusal.

OPM retains the shared [OPM/OMM elapsed-time profile](../design/validation-contract.md#opm-and-omm-elapsed-time-profile):
calendar-field-valid tokens are required even for MET/MRT. The full elapsed ordinal day
range in ODM §3.2.3.2 is not implemented by its public `CalendarEpoch` fields.
