# APM 2.0 conformance

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

The `apm` suite establishes:

| Concern | Evidence |
| --- | --- |
| KVN lexical and structural strictness | The APM marked-block scanner rejects non-ASCII/control input, overlong lines, malformed assignments, unknown and duplicate keywords, fixed-order violations within logical blocks, misplaced comments, unknown/nested blocks, and mismatched block ends. It accepts both the optional metadata delimiters and repeated attitude block families represented by the standard fixtures. |
| XML structure | The shared XML sequence engine is registered for the complete APM root, header, metadata, data, and six attitude logical-block families, including nested quaternion components. It rejects unknown, duplicate, and reordered children and non-schema attributes. |
| Valid input and preservation | All three shipped KVN fixtures and the shipped XML fixture parse through the public strict API. Generated KVN and XML reparse to the same typed model. |
| XML generation | XML generated from every shipped fixture validates against the official 4.0.0 master schema. Numeric domains have direct validation tests in addition to the [shared XSD oracle](../design/validation-contract.md#xsd-oracle-policy). |
| Shared surface contract | `library::parsing` and `library::output`, `conformance::generation`, the Python API tests, and family Criterion matrices provide common parsing, generation, diagnostics, dispatch, and workload evidence linked from the [conformance review index](README.md). |

The existing APM unit suite separately covers each attitude logical-block family, optional
quaternion/Euler derivatives, maneuver delta mass, the spin nutation choice and the values inside
both of its branches, root versioning, required metadata, and the requirement for at least one
attitude block.

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
| 48-62 | Spin block and mutually exclusive nutation descriptions | `SpinState`, conditional-choice tests plus value validation of both branches: non-empty reference frames, angle bounds on `SPIN_ALPHA`/`SPIN_DELTA`/`SPIN_ANGLE` and on the optional nutation and momentum angles, finite `SPIN_ANGLE_VEL` and `NUTATION_VEL`, and a non-negative `NUTATION_PER` |
| 63-72 | Inertia block | `InertiaState`, complete tensor tests |
| 73-82 | Maneuver block | `ApmManeuverParameters`, required torque/duration and optional mass tests |

No annex-A APM keyword or block is absent. APM's frame and time values are SANA-governed and so
remain caller-provided under the shared policy on [externally governed values](../design/validation-contract.md#registry-governed-values).

## Allocation and packaged-surface evidence

`fixed_family_allocations` fixes strict KVN parse budgets at 136 allocations/24,000 bytes and
preflighted streaming generation at 48 allocations/2,000 bytes for the shipped APM fixture.
`test_apm.py` plus the shared Python options matrix cover construction, setters, both
notations, files, and epochs. Strict binding audit, stubs/doc checks, wheel
verification, and Rust artifact verification are the packaged gates.

## Verification outcome

APM 2.0 meets the common [verification criteria](README.md#verification-criteria) for
self-contained format and model validation on Rust and Python. The requirement inventory,
family tests, shared public-route tests, and packaged tests are complementary evidence;
none alone establishes conformance or exhaustive mutation coverage.

## Decisions and limits

ADM annex J lists incompatible APM 1.0 differences: quaternion frame/direction keywords,
Euler sequence and inertia names, and the addition of repeatable logical-block delimiters.
A 2.0 message with a changed version header is not proof of legacy support. Input edition
1.0 is rejected until those actual grammar and semantic differences are implemented and reviewed.

Shared Rust/Python route coverage, including `test_conformance_matrix.py`, and installed
artifact checks are indexed once in the [common evidence table](README.md#shared-evidence).
