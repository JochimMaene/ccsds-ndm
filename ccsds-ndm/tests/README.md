# Rust tests

Run one family from the repository root with
`cargo test --manifest-path ccsds-ndm/Cargo.toml --test aem` (replace `aem` with any lowercase message family).
Run `just test` for the entire suite.
Run `cargo test --manifest-path ccsds-ndm/Cargo.toml --all-features` for Rust only.
Schema tests require `xmllint` (Debian/Ubuntu: `libxml2-utils`); the XSD numeric-facet
oracle also requires a JDK with Java source-file launching (Java 11 or newer).

The layout follows ownership, not the historical name of a test file:

- `<family>/`: one Cargo target per message family, including combined `ndm`.
- `library/`: shared detection, parsing/version contracts, XML strictness, errors,
  file readers, and typed/generic output contracts.
- `conformance/`: corpus parsing/generation, ODM 2 schema compatibility, and XSD oracles.
- `common/`: helpers used by multiple suites; not a test target.
- Top-level `*_allocations.rs`: one isolated executable per allocation budget.
  Each contains one test because its allocator counters are process-wide. Do not
  import these files into a family suite. Keeping Cargo's automatic discovery
  avoids explicit target registrations or wrapper files.

Run a shared suite with `--test library` or `--test conformance`; add a module
filter such as `--test conformance odm2` to run the ODM 2 cases.

Each message uses the same coverage checklist; case counts differ because the
formats have different rules.

Each family has one integration target. `parsing.rs` exercises the public readers;
`generation.rs` covers writers, roundtrips, and validation before output;
`model.rs` checks direct model operations. Keep specialized epoch, conversion,
validation, diagnostics, and minimal-message modules where they earn their own file. Shared
family fixtures belong in `main.rs`; helpers used once stay beside their test.
Tests of private parser functions and internal mechanics stay in `src/`.

| Guarantee | Where it belongs |
| --- | --- |
| Required fields, order, duplicates, unknown content, record widths | Family parsing tests |
| Numeric boundaries, conditional fields, choices, timeline rules | Model unit tests and family validation tests |
| Every shipped input is accepted, or rejected for its documented semantic defect | `conformance/parsing.rs` |
| Reference-fixture preservation and official XSD validation | `conformance/generation.rs`; AEM conversion, OEM/OPM/NDM generation |
| OPM/OEM/OMM edition 2 schema compatibility; numeric-facet oracle | `conformance/odm2.rs`; `conformance/schema.rs` |
| Notation-specific normalization, precision, or unrepresentable data | Family conversion/generation tests |
| Typed/generic dispatch, output, determinism, streaming, preflight, sink errors, atomic file conversion | `library/output.rs` |
| File parsing, explicit notation, missing files, invalid UTF-8 | `library/files.rs` |
| Detection, version policy, shared lexical regressions, displayed error context | `library/detection.rs`, `library/parsing.rs`, `library/errors.rs` |
| Atomic-write cleanup after a partial write fails | Unit test in `src/fsutil.rs` |
| XML root attributes, unknown metadata, forbidden characters, and wrapper rejection | `library/xml.rs` |
| Allocation growth | Separate `*_allocations.rs` binaries (global allocator instrumentation) |

Use `common::fixtures` for sorted corpus sweeps and `common::mutated` (or
`mutated_once`) for edits that must find their target. Use one valid baseline and
labelled mutations for related cases. Assert error codes and paths or typed error
fields where the API exposes them; use text only for uncoded diagnostics. Keep independently
wired fields and distinct valid/invalid boundaries covered. Shared API behavior
belongs in the shared contract; family tests retain notation-specific cases and
diagnostics. Roundtrip assertions compare complete models, with explicit expected
normalization where needed.

Reusing a fixture is not itself duplicate coverage: a dispatch check, a numeric
boundary, and an external schema check establish different guarantees. Do not add
another parse-only or basic roundtrip smoke test when those guarantees are already
covered. Keep a focused regression only when its input or assertion exercises a
distinct behavior. The corpus parse sweep also detects new fixtures that have not
yet been assigned a generation/normalization expectation.

The shared output contract covers all ten standalone families in both notations,
including interrupted/short writes and full sinks. Combined NDM has its own XML-only
case. Keep filesystem guarantees in the shared tests: create/replace, permissions,
in-place conversion, and preserving existing or absent destinations on failure.

The bundled standards and `docs/conformance/` inventories remain the authority
for the rules being tested. In particular, AEM interpolation sufficiency remains
unresolved; this test organization does not establish full compliance.
