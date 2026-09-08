# AEM, OEM, and OPM tests

Run `just conformance-aem`, `just conformance-oem`, or `just conformance-opm`.
Run `just test-rust` for the entire Rust suite.

Each message uses the same coverage checklist; case counts differ because the
formats have different rules.

OPM has four test modules: `parsing.rs`, `validation.rs`, `generation.rs`,
and `conversion.rs`. Its `main.rs` holds fixtures and shared diagnostic assertions.

| Guarantee | Where it belongs |
| --- | --- |
| Required fields, order, duplicates, unknown content, record widths | Family parsing tests |
| Numeric boundaries, conditional fields, choices, timeline rules | Model unit tests and family validation tests |
| Reference-fixture preservation and official XSD validation | AEM conversion, OEM generation, OPM conversion/generation |
| Notation-specific normalization, precision, or unrepresentable data | Family conversion/generation tests |
| Typed/generic output, determinism, streaming, preflight, sink errors, atomic file conversion | `message_output_contract.rs` |
| Input/depth/history limits | Family parsing/limits tests and shared `api_options.rs` / `family_contract.rs` |
| Allocation growth | Separate `*_allocations.rs` binaries (global allocator instrumentation) |

Use one valid baseline and labelled mutations for related cases. Keep independently
wired fields and distinct valid/invalid boundaries covered. Shared API behavior
belongs in the shared contract; family tests retain notation-specific cases and
diagnostics. Roundtrip assertions compare complete models, with explicit expected
normalization where needed.

The bundled standards and `docs/conformance/` inventories remain the authority
for the rules being tested. In particular, AEM interpolation sufficiency remains
unresolved; this test organization does not establish full compliance.
