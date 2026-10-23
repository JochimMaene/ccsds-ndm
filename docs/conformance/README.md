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
| TDM | [TDM 2.0](tdm-2.0.md) |
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
