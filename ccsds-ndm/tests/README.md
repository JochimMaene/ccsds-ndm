# Rust tests

Run one family from the repository root with
`cargo test --manifest-path ccsds-ndm/Cargo.toml --test aem` (replace `aem` with any lowercase message family).
Run `just test` for the entire suite.

Each message uses the same coverage checklist; case counts differ because the
formats have different rules.

Each family has one integration target with modules for its parsing, validation,
generation, and conversion behavior. Public model tests live alongside them; tests
requiring private implementation details stay in `src/`.

| Guarantee | Where it belongs |
| --- | --- |
| Required fields, order, duplicates, unknown content, record widths | Family parsing tests |
| Numeric boundaries, conditional fields, choices, timeline rules | Model unit tests and family validation tests |
| Reference-fixture preservation and official XSD validation | `family_generation_evidence.rs`; AEM conversion, OEM and OPM generation |
| Notation-specific normalization, precision, or unrepresentable data | Family conversion/generation tests |
| Typed/generic output, determinism, streaming, preflight, sink errors, atomic file conversion | `message_output_contract.rs` |
| XML nesting safety | `xml_root_contract.rs` |
| Allocation growth | Separate `*_allocations.rs` binaries (global allocator instrumentation) |

Use `common::fixtures` for sorted corpus sweeps and `common::mutated` (or
`mutated_once`) for edits that must find their target. Use one valid baseline and
labelled mutations for related cases. Assert error codes and paths or typed error
fields where the API exposes them; use text only for uncoded diagnostics. Keep independently
wired fields and distinct valid/invalid boundaries covered. Shared API behavior
belongs in the shared contract; family tests retain notation-specific cases and
diagnostics. Roundtrip assertions compare complete models, with explicit expected
normalization where needed.

The bundled standards and `docs/conformance/` inventories remain the authority
for the rules being tested. In particular, AEM interpolation sufficiency remains
unresolved; this test organization does not establish full compliance.
