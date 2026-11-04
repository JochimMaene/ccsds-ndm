# Conformance evidence

These records explain which CCSDS requirements the library enforces, how that behavior is
tested, and why it takes a particular interpretation when a book, example, or schema disagrees.
The [support matrix](../support-matrix.md) is the public statement of supported editions.
The [validation contract](../design/validation-contract.md) defines the common validation
boundaries and authority policy; family records contain only the decisions specific to that family.

## Verification criteria

Every current message family uses the same criteria. Verification applies to the stated
publication baseline, supported operations, and documented limits; it is not certification of
mission truth, interpolation algorithms, or an arbitrary external ICD.

1. **Requirements:** review the book's mandatory, optional, and conditional features against the
   typed model, parser, and root-reachable validators. Include populated optional blocks and
   repeated elements beyond the first.
2. **Input:** check required fields, alternatives, ordering, duplicates, unknown content, units,
   numeric domains, epochs, and cross-field rules in every supported notation and edition.
3. **Output:** check model preservation under the documented normalizations, deterministic generation, schema-valid XML, and
   explicit refusal where the chosen notation cannot represent the model. Test streaming
   preflight and failure paths as well as successful strings.
4. **Public surfaces:** exercise typed and generic Rust and Python parsing, validation, generation,
   files, conversion, diagnostics, and edited models. A shared dispatch test complements rather
   than replaces the family-specific rule tests.
5. **Artifacts:** run the Rust suite against the packaged crate and the Python suite against
   installed wheels, including a wheel built from the source distribution.
6. **Decisions:** record the governing publication/section and schema, the disagreement or
   ambiguity, the chosen behavior, its limitation, and an executable regression. A documented
   exception is part of the claim, not a missing rule concealed by a status label.

## Family records

| Family | Requirement and decision record |
| --- | --- |
| OPM | [OPM 3.0 and 2.0](opm-3.0.md) |
| OMM | [OMM 3.0 and 2.0](omm-3.0.md) |
| OEM | [OEM 3.0 and 2.0; legacy input limits](oem-3.0.md) |
| OCM | [OCM 3.0](ocm-3.0.md) |
| CDM | [CDM 1.0](cdm-1.0.md) |
| TDM | [TDM 2.0; restricted 1.0 KVN input](tdm-2.0.md) |
| RDM | [RDM 1.0](rdm-1.0.md) |
| APM | [APM 2.0](apm-2.0.md) |
| AEM | [AEM 2.0](aem-2.0.md) |
| ACM | [ACM 2.0](acm-2.0.md) |
| Combined NDM | [XML envelope](combined-ndm.md) |

## Shared evidence

| Guarantee | Executable evidence |
| --- | --- |
| Explicit edition/notation policy, typed and generic readers | Rust `library::parsing`, `library::detection`, `library::files` |
| Root structure, namespaces, trailing content, equivalent UTF-8 declarations, and a leading BOM | Rust `library::xml`, `library::detection`, and family parsing suites |
| Typed/generic output equality, semantic validation, deterministic strings, streaming preflight, short/interrupted writes and sink failures | Rust `library::output` |
| Corpus preservation under documented normalizations, notation-specific refusals, and official XML schemas | Rust `conformance::generation`, `conformance::odm2`, `conformance::schema`, and family generation/conversion suites |
| Every current family/edition through typed/generic Python strings, files, validation and conversion in both notations where defined | Python `test_conformance_matrix.py` |
| Live Python children, edited numeric values, constructor contracts and structured exceptions | Python family tests, `test_api_consistency.py`, and `tests/typing/` |
| Installed artifacts use the same implementation | `scripts/verify-rust-package.sh` and `scripts/verify-python-package.sh`; run `just verify` |
| Resource growth and measured workloads | Isolated Rust `*_allocations` suites, KVN/XML family benchmark matrices, and the Python OEM construction benchmark |

The runtime does not invoke an XSD engine or consult a registry over the network. Schema tests
provide an independent structural oracle; direct validators cover semantics the schema cannot
express. Numeric-facet oracle limitations are recorded in the
[validation contract](../design/validation-contract.md#xsd-oracle-policy).

## Maintaining the evidence

Change the family record alongside a rule or its interpretation, and update the support matrix
only after the affected criteria pass. Record the publication baseline explicitly, including
corrigenda: an unchanged message version number does not mean that later corrigenda add no rules.
Keep published examples distinguishable from corrected test inputs. Performance observations are
dated measurements, not conformance requirements or portable speed guarantees.

## Release review resolutions

The shared authority policy is in the validation contract; family records hold the remaining
field-specific decisions. The following checklist connects the confirmed release defects and
compatibility decisions to their executable evidence.

| Resolved issue or decision | Governing rule and resulting behavior | Regression or maintained record |
| --- | --- | --- |
| Python nested/cyclic combined messages | NDM/XML 4.11 allows standalone constituents, not recursive envelopes; conversion rejects nested Python objects before descending. | Python `test_combined_messages_reject_nested_or_cyclic_constituents_before_conversion` |
| Combined source coordinates | Diagnostic positions describe the original document, including constituent parsing with inherited namespaces. | Rust NDM `constituent_error_locations_refer_to_the_original_combined_document` |
| ACM XML categories | XSD `attBasisType` and `adMethodType` govern exact XML strings; KVN aliases/custom text retain their documented separate domain. | ACM generation and epoch suites; [ACM record](acm-2.0.md) |
| ACM invalid metadata epochs | ADM 6.8.9 requires real absolute calendar fields; relative tags must contain digits, and zero/negative offsets remain supported. | ACM epoch suite |
| CDM fixed KVN units | CDM 6.2.4.1 requires applicable units on output; absent annotations remain accepted on input. | CDM generation suite; [CDM record](cdm-1.0.md) |
| CDM missing-value normalization | A supplied incompatible unit cannot be hidden by empty/nil/N/A text; decoded empty CDATA follows the same policy. Optional empty/N/A containers normalize without fabricating data. | CDM optional-value and parsing suites |
| RDM impact-altitude units | XSD `altType` permits absent XML units; KVN still supplies metres. | RDM parsing/generation and Python editing regressions |
| Repeated/empty user-defined names | Ordered duplicate names are preserved, including OPM. A bare `USER_DEFINED_` suffix is rejected; opaque empty/N/A values remain payloads. | OPM conversion and shared library parsing suites |
| Doppler versus phase-count output | TDM 4.3.2 bounds KVN integer Doppler counts; 4.3.11 retains arbitrary-precision phase counts. Negative zero is canonicalized on KVN output. | TDM generation suite |
| XML unsigned integer zero | XSD integer value equality includes `-0`; native `u32` bounds still apply. | Shared parsing `unsigned_xml_integer_zero_uses_xsd_value_equality`; OMM schema controls |
| Python row replacement and names | ACM rows derive their count atomically; AEM object-name comparison recognizes blank/underscore equivalence. | Python ACM and Rust AEM validation suites |
| Source names, units, and API signatures | Field-specific readers identify the keyword and source record; `OpmCovarianceMatrix(comment=...)` matches its property. | APM/ACM/OEM diagnostic suites, Python OPM regression, generated stubs |
| Common producer spellings | Every family accepts an unterminated final KVN record and printable UTF-8 comments. Zero-offset epoch aliases normalize to Z without a time-scale conversion. | Shared library parsing suite; [lexical contract](../design/validation-contract.md#shared-lexical-decisions) |
| Book conditionals absent from XSD | Missing ACM frames, OEM interpolation degree, and OPM maneuver mass remain errors; schema optionality does not remove explicit book dependencies. | Family records and invalid-model generation tests |
| OCM omitted defaults | ODM 6.2.1.3 explicitly permits omission when its defaults apply. No writer change is required for orbit-revolution basis, interpolation degree, or OEB parent frame. | [OCM defaults](ocm-3.0.md#book-defaults-and-producer-boundaries) |
| OEM usable overlap | ODM 5.2.4.4 forbids overlap except at a shared endpoint; this is normative, not extra strictness. Total spans may overlap. | OEM validation suite and [OEM record](oem-3.0.md) |
| Legacy editions and malformed feeds | Older combined envelopes, TDM 1.0 XML, and attitude 1.0 remain unsupported pending edition-specific implementation/evidence. Undeclared prefixes and missing required provenance remain errors. | [Support matrix](../support-matrix.md#legacy-compatibility), versioning and envelope tests |
| Precision, physics, and external context | The 16-fraction-digit profile and 64-byte epoch storage bound remain explicit. DOR/VLBI reference context, external registries, interpolation feasibility and physical truth have the caller boundaries listed by each family. | [Validation contract](../design/validation-contract.md), [TDM record](tdm-2.0.md) |

Documentation corrections also cover the OMM coefficient/theory requirements, TDM 1.0 input
property, ACM custom-method extension, OCM relative usable bounds, OEM covariance comments,
ACM arbitrary line lengths, signed-zero normalization, missing-value codes, live/snapshot Python
editing, and lossy source-book extracts. These statements live beside their governing rules
rather than introducing another parser mode or duplicate support matrix.
