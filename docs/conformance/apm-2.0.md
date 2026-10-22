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
| KVN lexical and structural strictness | The APM marked-block scanner rejects non-ASCII/control input, malformed assignments, unknown and duplicate keywords, fixed-order violations within logical blocks, misplaced comments, unknown/nested blocks, and mismatched block ends. It accepts repeated attitude block families represented by the standard fixtures. Reading also tolerates `META_START`/`META_STOP` around the metadata, a non-standard producer spelling that ADM table 3-2 does not define; the writer never emits them (`apm::generation::apm_kvn_output_omits_metadata_delimiters_and_keeps_quaternion_rate_units`). |
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

**Line endings and duration units.** ADM §6.6.6 permits CR, LF, CRLF and LFCR.
APM uses the shared byte-length-preserving line normalization, covered by
`apm::parsing::apm_accepts_all_adm_line_endings`. ADM table 3-3 and the XSD
`durationType`/`timeUnits` define `NUTATION_PER` in seconds. Days are rejected in both
readers, edited models and generation, rather than silently reinterpreted as seconds;
`apm::generation::apm_nutation_period_is_seconds_without_reinterpreting_day_units` covers that boundary.


ADM annex J lists incompatible APM 1.0 differences: quaternion frame/direction keywords,
Euler sequence and inertia names, and the addition of repeatable logical-block delimiters.
A 2.0 message with a changed version header is not proof of legacy support. Input edition
1.0 is rejected until those actual grammar and semantic differences are implemented and reviewed.

Shared Rust/Python route coverage, including `test_conformance_matrix.py`, and installed
artifact checks are indexed once in the [common evidence table](README.md#shared-evidence).

**Angle domain.** ADM §6.8.1 gives angles in degrees "between −360 and 360"; the XSD
`angleRange` makes this `[-360, 360)` (upper bound exclusive). The XSD interval applies in KVN,
XML and model validation alike, so a valid model always converts (the shared rule on
[XSD facets](../design/validation-contract.md#authority-and-representability)). Radians require an
exchange-partner ICD and are outside this supported profile; unit annotations are never ignored
on dimensionless quaternion components.

**Numbers.** KVN number reading and 16-significant-digit writing follow the shared
[KVN number rule](../design/validation-contract.md#shared-lexical-decisions). ADM §6.8.5 says
`-0` is not a supported value. That sentence sits in the KVN chapter, and the KVN writer already
writes negative zero as `0.0`. XML follows the XSD `xsd:double`, which admits `-0`, so XML keeps the
sign ([preservation rules](../design/validation-contract.md#preservation-and-normalization)).

**Units on output.** KVN output keeps every unit annotation the model carries, including
`[1/s]` on `Q1_DOT`..`QC_DOT` (table 3-3).

**Quaternion derivatives are all-or-none.** Table 3-3 marks each of `Q1_DOT`, `Q2_DOT`, `Q3_DOT`
and `QC_DOT` optional on its own. The XSD `quaternionDotType` requires all four together. The
library follows the XSD in every notation, so a partial set is rejected rather than a message being
produced that XML cannot represent.

**Rotation sequences.** XML accepts only the twelve uppercase `rotseqType` letters. KVN and Python
also read lowercase (`zxz`, ADM §6.8.6) and, as a documented legacy alias, the ADM 1.0 numeric
spellings (`313` for `ZXZ`, annex J), because both convert without loss. Every writer emits the
uppercase letters (`apm::parsing::euler_rot_seq_reads_lowercase_and_legacy_digits_in_kvn_only`).

**Logical-block order.** Table 3-3 and the XSD `apmData` sequence list the blocks in a fixed
order (quaternion, Euler, angular velocity, spin, inertia, maneuver). KVN reading accepts the
blocks in any order and interleaved, since each block is self-delimited and no value is lost.
The model stores each block type in its own list, so the order within one block type is kept but
the relative order across block types is not: output is always written in table order.

**Comments.** XML allows a line break inside one `<COMMENT>`; KVN has one comment per line. On
KVN output such a comment becomes several `COMMENT` lines, which read back as several comment
entries, so the model differs after an XML-to-KVN hop. Without metadata delimiters, KVN comments
after the header belong to the metadata and comments after `TIME_SYSTEM` to the data section
(ADM 6.10.2; `apm::parsing::apm_comments_without_metadata_delimiters_stay_in_their_section`).

## Spin choice: schema versus prose

ADM §3.2.4.6 says a spin block contains either the complete nutation triple or the complete
momentum triple. Bundled `spinStateType` instead sets the choice's `minOccurs` to zero.
The library follows the schema in both notations: neither triple is allowed, one complete
triple is allowed, and partial or simultaneous triples fail. It does not invent nutation
or momentum values to complete a bare spin state. The regression
`apm::generation::spin_without_optional_triples_follows_the_schema` checks preservation and schema validity.
