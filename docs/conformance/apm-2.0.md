# APM 2.0 Focused Conformance Evidence

This inventory records maintainer evidence for standalone APM 2.0. The
[support matrix](../support-matrix.md) is the user-facing statement of current support.

## Scope and authorities

- Message edition: APM 2.0.
- Semantic and KVN authority: CCSDS 504.0-B-2, principally section 3 and the APM
  implementation-conformance statement in annex A.
- XML authority: NDM/XML schema set 4.0.0 and APM schema 2.0.
- Profile: strict standalone parsing, self-contained validation, deterministic KVN/XML generation,
  and notation conversion through the shared Rust generation gate.

## Executable evidence

`just conformance-apm` runs `apm_conformance`, which establishes:

| Concern | Evidence |
| --- | --- |
| KVN lexical and structural strictness | The APM marked-block scanner rejects non-ASCII/control input, overlong lines, malformed assignments, unknown and duplicate keywords, fixed-order violations within logical blocks, misplaced comments, unknown/nested blocks, and mismatched block ends. It accepts both the optional metadata delimiters and repeated attitude block families represented by the standard fixtures. |
| XML structure | The shared XML sequence engine is registered for the complete APM root, header, metadata, data, and six attitude logical-block families, including nested quaternion components. It rejects unknown, duplicate, and reordered children and non-schema attributes. |
| Valid input and preservation | All three shipped KVN fixtures and the shipped XML fixture parse through the public strict API. Generated KVN and XML reparse to the same typed model. |
| XML generation | XML generated from every shipped fixture validates against the official 4.0.0 master schema. |
| Shared resource and surface contract | `family_contract`, `family_generation_evidence`, the Python options tests, CLI dispatch test, and family Criterion matrices provide the common bounded parsing/generation, diagnostics, dispatch, and workload evidence linked from `family-shared-contract.md`. |

The existing APM unit suite separately covers each attitude logical-block family, optional
quaternion/Euler derivatives, maneuver delta mass, the spin nutation choice, root versioning,
required metadata, and the requirement for at least one attitude block.

## Complete ICS reconciliation

All 82 numbered items in CCSDS 504.0-B-2 annex A2.2.1 were reconciled against the model and
executable evidence:

| ICS items | Logical requirement set | Reconciled implementation/evidence |
| --- | --- | --- |
| 1-6 | Header/version fields | `Apm`, `AdmHeader`, root/version and calendar-epoch tests |
| 7-11 | Metadata | `ApmMetadata`, required-field and strict sequence tests |
| 12-13 | Data comment and state epoch | `ApmData`, absolute-epoch and roundtrip tests |
| 14-26 | Quaternion block and optional derivatives | `QuaternionState`, complete block/choice tests |
| 27-38 | Euler block and optional derivatives | `EulerAngleState`, sequence and optional-field tests |
| 39-47 | Angular-velocity block | `AngVelState`, frame/three-component tests |
| 48-62 | Spin block and mutually exclusive nutation descriptions | `SpinState`, conditional-choice tests |
| 63-72 | Inertia block | `InertiaState`, complete tensor tests |
| 73-82 | Maneuver block | `ManeuverParameters`, required torque/duration and optional mass tests |

No annex-A APM keyword or block is absent. External SANA frame/time values remain caller-provided
strings rather than a bundled, time-sensitive registry snapshot.

## Allocation and packaged-surface evidence

`fixed_family_allocations` fixes strict KVN parse budgets at 136 allocations/24,000 bytes and
preflighted streaming generation at 48 allocations/2,000 bytes for the shipped APM fixture.
`family_surface_cli` covers binary validation, conversion, identity, and zero-byte output-limit
failure. `test_apm.py` plus the shared Python options matrix cover construction, setters, both
notations, files, epochs, and resource limits. Strict binding audit, stubs/doc checks, wheel
verification, and Rust artifact verification are the packaged gates.

## Reproducible performance observation

The following commands measure the same public parse and generation paths used by consumers:

```text
cargo bench --manifest-path ccsds-ndm/Cargo.toml --bench kvn_benches -- 'kvn_message_matrix/(parse|generate)/apm' --sample-size 20 --measurement-time 2 --warm-up-time 1
cargo bench --manifest-path ccsds-ndm/Cargo.toml --bench xml_benches -- 'xml_message_matrix/(parse|generate)/apm' --sample-size 20 --measurement-time 2 --warm-up-time 1
```

On 2026-07-18 the observed 95% intervals were 10.188-11.472 us for KVN parse,
4.205-5.051 us for KVN generation, 33.382-40.183 us for XML parse, and 5.888-7.064 us for XML
generation. These are observations, not unexplained release thresholds; the registered family
benchmarks provide the CodSpeed comparison surface for subsequent changes.

## Remaining verification work

APM remains available rather than verified. Its complete ICS feature inventory and packaged
surfaces are reconciled; final message-level review remains.
